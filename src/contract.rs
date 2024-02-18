use crate::crypto::verify_arbitrary;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, SignedData};
use crate::state::{Campaign, ADMIN, CAMPAIGN_POOL, NONCES, PUBKEY};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Addr, BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut, Empty, Env, MessageInfo,
    Response, StdError, StdResult, Uint128,
};
use cw2::{get_contract_version, set_contract_version};
use semver::Version;

const CONTRACT_NAME: &str = "crates.io:reward_pool";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, StdError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    PUBKEY.save(deps.storage, &msg.pubkey)?;
    ADMIN.save(
        deps.storage,
        &(deps.api.addr_canonicalize(info.sender.as_str()))?,
    )?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: Empty) -> Result<Response, StdError> {
    let new_version: Version = CONTRACT_VERSION.parse().unwrap();
    let current_version = get_contract_version(deps.storage)?;

    if current_version.contract != CONTRACT_NAME {
        return Err(StdError::generic_err(
            "Can only upgrade from same contract type",
        ));
    }

    if current_version.version.parse::<Version>().unwrap() >= new_version {
        return Err(StdError::generic_err(
            "Cannot upgrade from a newer contract version",
        ));
    }

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new().add_attribute("method", "migrate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, StdError> {
    match msg {
        ExecuteMsg::EditAdmin { new_admin } => edit_admin(deps, info, new_admin),
        ExecuteMsg::Deposit { campaign_id } => deposit(deps, env, info, campaign_id),
        ExecuteMsg::Claim {
            campaign_id,
            amount,
            denom,
            nonce,
            signature,
        } => claim(
            deps,
            env,
            info,
            campaign_id,
            denom,
            amount,
            nonce,
            signature,
        ),
        ExecuteMsg::Withdraw { amount } => withdraw(deps, env, info, amount),
        ExecuteMsg::Cancel { campaign_id } => cancel(deps, env, info, campaign_id),
        ExecuteMsg::SetCpool {
            campaign_id,
            amount,
        } => set_cpool(deps, env, info, campaign_id, amount),
    }
}

pub fn edit_admin(deps: DepsMut, info: MessageInfo, new_admin: Addr) -> Result<Response, StdError> {
    let admin = ADMIN.load(deps.storage)?;
    if admin != deps.api.addr_canonicalize(info.sender.as_str())? {
        return Err(StdError::generic_err("This function is for Admin only"));
    }

    ADMIN.save(
        deps.storage,
        &deps.api.addr_canonicalize(new_admin.as_str())?,
    )?;

    Ok(Response::new())
}

pub fn deposit(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    campaign_id: String,
) -> Result<Response, StdError> {
    let native_denom = deps.querier.query_bonded_denom()?;
    let mut funds = info.funds.clone();
    let coin = funds.pop();

    if funds.len() > 1 {
        return Err(StdError::generic_err("Only one coin is allowed"));
    }

    let amount_sent = match coin {
        Some(ref coin) => coin.amount,
        None => return Err(StdError::generic_err("No funds were sent")),
    };

    if coin.unwrap().denom != native_denom {
        return Err(StdError::generic_err("Invalid denom"));
    }

    match CAMPAIGN_POOL.may_load(deps.storage, campaign_id.clone())? {
        Some(mut campaign) => {
            campaign.amount += amount_sent;
            CAMPAIGN_POOL.save(deps.storage, campaign_id.clone(), &campaign)?;
        }
        None => CAMPAIGN_POOL.save(
            deps.storage,
            campaign_id.clone(),
            &Campaign {
                owner: info.sender,
                amount: amount_sent,
            },
        )?,
    };

    Ok(Response::new().add_attribute("method", "deposit"))
}

#[allow(clippy::too_many_arguments)]
pub fn claim(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    campaign_id: String,
    denom: String,
    amount: Uint128,
    nonce: String,
    signature: Binary,
) -> Result<Response, StdError> {
    // Check if nonce has been used
    if NONCES.has(deps.storage, &nonce) {
        return Err(StdError::generic_err("Nonce has been used"));
    } else {
        NONCES.save(deps.storage, &nonce, &true)?;
    }

    // Check if data is correctly signed
    verify_arbitrary(
        deps.as_ref(),
        &SignedData {
            campaign_id: campaign_id.clone(),
            amount,
            denom: denom.clone(),
            nonce: nonce.clone(),
            sender: info.sender.clone(),
        },
        &signature,
    )?;

    // Check if campaign exists and has funds
    if let Some(data) = CAMPAIGN_POOL.may_load(deps.storage, campaign_id.clone())? {
        let mut campaign = data;
        if campaign.amount > amount {
            campaign.amount -= amount;
            CAMPAIGN_POOL.save(deps.storage, campaign_id, &campaign)?;
        } else {
            return Err(StdError::generic_err("Campaign does not have enough funds"));
        }
    } else {
        return Err(StdError::generic_err("Campaign ID not valid"));
    }

    // Check if denom requested is used by contract
    let native_denom = deps.querier.query_bonded_denom()?;
    if native_denom != denom {
        return Err(StdError::generic_err("Invalid denom"));
    }

    // Send funds
    Ok(Response::new()
        .add_attribute("method", "claim")
        .add_message(CosmosMsg::Bank(BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: vec![Coin {
                denom: denom.clone(),
                amount,
            }],
        })))
}

pub fn withdraw(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, StdError> {
    let admin = ADMIN.load(deps.storage)?;

    if deps.api.addr_canonicalize(info.sender.as_str())? != admin {
        return Err(StdError::generic_err("Only contract owner can withdraw"));
    }

    let native_denom = deps.querier.query_bonded_denom()?;

    let own_balance: Uint128 = deps
        .querier
        .query_balance(env.contract.address, native_denom.clone())
        .unwrap_or_default()
        .amount;

    if amount > own_balance {
        return Err(StdError::generic_err("Not enough funds in the contract"));
    }
    let to_address = deps.api.addr_humanize(&admin)?.to_string();

    let res = Response::new()
        .add_attribute("method", "withdraw")
        .add_message(CosmosMsg::Bank(BankMsg::Send {
            to_address,
            amount: vec![Coin {
                denom: native_denom.clone(),
                amount,
            }],
        }));

    Ok(res)
}

pub fn cancel(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    campaign_id: String,
) -> Result<Response, StdError> {
    let admin = ADMIN.load(deps.storage)?;

    match CAMPAIGN_POOL.may_load(deps.storage, campaign_id.clone())? {
        Some(campaign) => {
            if deps.api.addr_canonicalize(info.sender.as_str())? != admin
                && info.sender != campaign.owner
            {
                return Err(StdError::generic_err(
                    "Only campaign owner can cancel the campaign",
                ));
            }

            if campaign.amount < Uint128::one() {
                CAMPAIGN_POOL.remove(deps.storage, campaign_id);
                return Ok(Response::new().add_attribute("method", "cancel"));
            }

            let amount = campaign.amount;

            let native_denom = deps.querier.query_bonded_denom()?;
            let res = Response::new()
                .add_attribute("method", "cancel")
                .add_message(CosmosMsg::Bank(BankMsg::Send {
                    to_address: campaign.owner.clone().to_string(),
                    amount: vec![Coin {
                        denom: native_denom.clone(),
                        amount,
                    }],
                }));

            CAMPAIGN_POOL.remove(deps.storage, campaign_id);

            Ok(res)
        }
        None => Err(StdError::generic_err("Campaign does not exist")),
    }
}

pub fn set_cpool(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    campaign_id: String,
    amount: Uint128,
) -> Result<Response, StdError> {
    let admin = ADMIN.load(deps.storage)?;

    if deps.api.addr_canonicalize(info.sender.as_str())? != admin {
        return Err(StdError::generic_err(
            "Only contract owner can set the campaign pool",
        ));
    }

    match CAMPAIGN_POOL.may_load(deps.storage, campaign_id.clone())? {
        Some(mut campaign) => {
            campaign.amount = amount;
            CAMPAIGN_POOL.save(deps.storage, campaign_id, &campaign)
        }
        None => CAMPAIGN_POOL.save(
            deps.storage,
            campaign_id,
            &Campaign {
                owner: info.sender,
                amount,
            },
        ),
    }?;

    Ok(Response::new().add_attribute("method", "set_cpool"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCpool { campaign_id } => query_campaign_pool(deps, env, campaign_id),
    }
}

pub fn query_campaign_pool(deps: Deps, _env: Env, campaign_id: String) -> StdResult<Binary> {
    let campaign_pool = CAMPAIGN_POOL.may_load(deps.storage, campaign_id)?;

    match campaign_pool {
        Some(pool) => to_json_binary(&pool),
        None => Err(StdError::generic_err("Campaign does not exist")),
    }
}
