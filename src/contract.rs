#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{DepsMut, Env, Response, MessageInfo, StdError, CosmosMsg, BankMsg, Coin, Uint128, Deps, StdResult, Binary, to_binary};
use crate::state::{Campaign, State, STATE, UserPool};
use std::collections::HashMap;
use crate::msg::{CampaignCheckRequest, CampaignCheckResponse, ExecuteMsg, InstantiateMsg, QueryMsg, UserRewardRequest, UserRewardResponse};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, StdError> {

    let state = State {
        owner: deps.api.addr_canonicalize(info.sender.as_str())?,
        campaign_pool: HashMap::new(),
        user_pool: HashMap::new(),
        creation_fee: Uint128::zero(),
    };

    STATE.save(deps.storage, &state)?;

    return Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, StdError> {

    match msg {
        ExecuteMsg::Deposit { campaign_id } => deposit(
            deps,
            env,
            info,
            campaign_id,
        ),
        ExecuteMsg::RewardAll { user_rewards } => reward_all(
            deps,
            env,
            info,
            user_rewards,
        ),
        ExecuteMsg::Claim { campaign_id } => claim(
            deps,
            env,
            info,
            campaign_id,
        ),
        ExecuteMsg::Check { requests } => check(
            deps,
            env,
            info,
            requests,
        ),
        ExecuteMsg::Withdraw { amount } => withdraw(
            deps,
            env,
            info,
            amount,
        ),
        ExecuteMsg::WithdrawFee {} => withdraw_fee(
            deps,
            env,
            info,
        ),
        ExecuteMsg::SetRefundable { campaign_id } => set_refundable(
            deps,
            env,
            info,
            campaign_id,
        ),
        ExecuteMsg::Cancel { campaign_id } => cancel(
            deps,
            env,
            info,
            campaign_id,
        ),
        ExecuteMsg::SetCpool { campaign_id,amount  } => set_cpool(
            deps,
            env,
            info,
            campaign_id,
            amount,
        ),
        ExecuteMsg::SetUpool { user_address, reward_pool_id, amount  } => set_upool(
            deps,
            env,
            info,
            user_address,
            reward_pool_id,
            amount
        ),
    }
}

pub fn deposit(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    campaign_id: String,
) -> Result<Response, StdError> {
    let mut state = STATE.load(deps.storage)?;

    let amount_sent = match info.funds.get(0) {
        Some(coin) => coin.amount,
        None => return Err(StdError::generic_err("No funds were sent")),
    };

    if let Some(campaign) = state.campaign_pool.get_mut(&campaign_id) {
        campaign.amount += amount_sent;
    } else {
        state.campaign_pool.insert(
            campaign_id.clone(),
            Campaign {
                owner: info.sender,
                amount: amount_sent,
                refundable: false,
            },
        );
    }

    STATE.save(deps.storage, &state)?;

    return Ok(Response::new()
        .add_attribute("method", "deposit")
    );
}

pub fn reward_all(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    user_rewards: Vec<UserRewardRequest>,
) -> Result<Response, StdError> {
    let mut state = STATE.load(deps.storage)?;

    if deps.api.addr_canonicalize(info.sender.as_str())? != state.owner {
        return Err(StdError::generic_err("Only contract owner can call this function"));
    }

    let mut res = vec![];

    for request in user_rewards {
        if let Some(campaign) = state.campaign_pool.get_mut(&request.campaign_id) {
            let user_pool_id = format!("{}_{}", request.user_address, request.campaign_id);

            let flag: bool = campaign.amount > request.amount;

            if flag {
                if let Some(user_pool) = state.user_pool.get_mut(&user_pool_id) {
                    user_pool.amount += request.amount;
                    campaign.amount -= request.amount;
                } else {
                    state.user_pool.insert(
                        user_pool_id.clone(),
                        UserPool {
                            amount: request.amount,
                        },
                    );
                    campaign.amount -= request.amount;
                }
            }

            res.push(UserRewardResponse {
                campaign_id: request.campaign_id.clone(),
                user_address: request.user_address.clone(),
                status: flag,
            });

        } else {
            return Err(StdError::generic_err("Campaign does not exist"));
        }
    }
    STATE.save(deps.storage, &state)?;

    return Ok(Response::new()
        .add_attribute("method", "reward_all")
        .set_data(to_binary(&res).unwrap())
    );
}

pub fn claim(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    campaign_id: String,
) -> Result<Response, StdError>  {
    let mut state = STATE.load(deps.storage)?;
    let mut amount = Uint128::zero();

    let user_pool_id = format!("{}_{}", info.sender.to_string(), campaign_id);

    if let Some(user_pool) = state.user_pool.get_mut(&user_pool_id) {
        amount = user_pool.amount;

        if amount < Uint128::one() {
            state.user_pool.remove(&user_pool_id);
            return Err(StdError::generic_err("Nothing to claim"));
        }

    } else {
        return Err(StdError::generic_err("User pool does not exist"));
    }

    state.user_pool.remove(&user_pool_id);

    STATE.save(deps.storage, &state)?;

    let bond_denom = deps.querier.query_bonded_denom()?;
    let res = Response::new()
        .add_attribute("method", "claim")
        .add_message(CosmosMsg::Bank(BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: vec![Coin { denom: bond_denom.clone(), amount }],
        }));

    return Ok(res);
}

pub fn check(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    requests: Vec<CampaignCheckRequest>,
) -> Result<Response, StdError> {
    let mut state = STATE.load(deps.storage)?;

    if deps.api.addr_canonicalize(info.sender.as_str())? != state.owner {
        return Err(StdError::generic_err("Only contract owner can call this function"));
    }

    let mut res = vec![];

    for request in requests {
        if let Some(campaign) = state.campaign_pool.get_mut(&request.campaign_id) {
            let delta = campaign.amount.checked_sub(request.amount);
            if delta.is_err() {
                return Err(StdError::generic_err("Provided amount is greater than the current campaign amount"));
            }

            state.creation_fee += delta.unwrap();

            res.push(CampaignCheckResponse {
                campaign_id: request.campaign_id.clone(),
                owner: campaign.owner.clone().to_string(),
                amount_before_deduction: campaign.amount,
            });

            campaign.amount = request.amount;
        } else {
            return Err(StdError::generic_err("Campaign does not exist"));
        }
    }

    STATE.save(deps.storage, &state)?;

    return Ok(Response::new()
        .add_attribute("method", "reward_all")
        .set_data(to_binary(&res).unwrap())
    );
}

pub fn withdraw(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, StdError> {
    let state = STATE.load(deps.storage)?;

    if deps.api.addr_canonicalize(info.sender.as_str())? != state.owner {
        return Err(StdError::generic_err("Only contract owner can withdraw"));
    }

    let bond_denom = deps.querier.query_bonded_denom()?;

    let own_balance: Uint128 = deps
        .querier
        .query_balance(&env.contract.address, bond_denom.clone())
        .unwrap_or_default()
        .amount;

    if amount > own_balance {
        return Err(StdError::generic_err("Not enough funds in the contract"));
    }
    let to_address = deps.api.addr_humanize(&state.owner)?.to_string();

    let res = Response::new()
        .add_attribute("method", "withdraw")
        .add_message(CosmosMsg::Bank(BankMsg::Send {
            to_address,
            amount: vec![Coin { denom: bond_denom.clone(), amount }],
        }));

    return Ok(res)
}

pub fn withdraw_fee(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
) -> Result<Response, StdError> {
    let mut state = STATE.load(deps.storage)?;

    if deps.api.addr_canonicalize(info.sender.as_str())? != state.owner {
        return Err(StdError::generic_err("Only contract owner can withdraw"));
    }

    let amount = state.creation_fee;
    state.creation_fee = Uint128::zero();

    STATE.save(deps.storage, &state)?;
    let bond_denom = deps.querier.query_bonded_denom()?;
    let to_address = deps.api.addr_humanize(&state.owner)?.to_string();

    let res = Response::new()
        .add_attribute("method", "withdraw_fee")
        .add_message(CosmosMsg::Bank(BankMsg::Send {
            to_address,
            amount: vec![Coin { denom: bond_denom.clone(), amount }],
        }));

    return Ok(res)
}

pub fn set_refundable(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    campaign_id: String,
) -> Result<Response, StdError> {
    let mut state = STATE.load(deps.storage)?;

    if deps.api.addr_canonicalize(info.sender.as_str())? != state.owner {
        return Err(StdError::generic_err("Only contract owner can make the campaign refundable"));
    }

    return if let Some(campaign) = state.campaign_pool.get_mut(&campaign_id) {
        campaign.refundable = true;
        STATE.save(deps.storage, &state)?;

        return Ok(Response::new()
            .add_attribute("method", "set_refundable")
        );
    } else {
        Err(StdError::generic_err("Campaign does not exist"))
    }
}

pub fn cancel(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    campaign_id: String,
) -> Result<Response, StdError> {
    let mut state = STATE.load(deps.storage)?;

    if let Some(campaign) = state.campaign_pool.get_mut(&campaign_id) {

        if deps.api.addr_canonicalize(info.sender.as_str())? != state.owner || info.sender != campaign.owner {
            return Err(StdError::generic_err("Only campaign owner can cancel the campaign"));
        }

        if !campaign.refundable {
            return Err(StdError::generic_err("Campaign was not set to be refundable"));
        }

        if campaign.amount < Uint128::one() {
            state.campaign_pool.remove(&campaign_id);
            return Ok(Response::new()
                .add_attribute("method", "cancel"));
        }

        let amount = campaign.amount;

        let bond_denom = deps.querier.query_bonded_denom()?;
        let res = Response::new()
            .add_attribute("method", "cancel")
            .add_message(CosmosMsg::Bank(BankMsg::Send {
                to_address: campaign.owner.clone().to_string(),
                amount: vec![Coin { denom: bond_denom.clone(), amount }],
            }));

        state.campaign_pool.remove(&campaign_id);
        STATE.save(deps.storage, &state)?;

        return Ok(res)
    } else {
        Err(StdError::generic_err("Campaign does not exist"))
    }
}
pub fn set_cpool(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    campaign_id: String,
    amount: Uint128,
) -> Result<Response, StdError> {
    let mut state = STATE.load(deps.storage)?;

    if deps.api.addr_canonicalize(info.sender.as_str())? != state.owner {
        return Err(StdError::generic_err("Only contract owner can set the campaign pool"));
    }

    if let Some(campaign) = state.campaign_pool.get_mut(&campaign_id) {
        campaign.amount = amount;
    } else {
        state.campaign_pool.insert(
            campaign_id.clone(),
            Campaign {
                owner: info.sender,
                amount,
                refundable: false,
            },
        );
    }
    STATE.save(deps.storage, &state)?;

    return Ok(Response::new()
        .add_attribute("method", "set_cpool")
    );
}

pub fn set_upool(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    user_address: String,
    reward_pool_id: String,
    amount: Uint128,
) -> Result<Response, StdError> {
    let mut state = STATE.load(deps.storage)?;

    if deps.api.addr_canonicalize(info.sender.as_str())? != state.owner {
        return Err(StdError::generic_err("Only contract owner can set the user pool"));
    }

    let user_pool_id = format!("{}_{}", user_address, reward_pool_id);

    if let Some(user_pool) = state.user_pool.get_mut(&user_pool_id) {
        user_pool.amount = amount;
    } else {
        state.user_pool.insert(
            user_pool_id.clone(),
            UserPool {
                amount,
            },
        );

        return Err(StdError::generic_err("User pool does not exist"))
    }
    STATE.save(deps.storage, &state)?;

    return Ok(Response::new()
        .add_attribute("method", "set_upool")
    );
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCpool { campaign_id } => query_campaign_pool(deps, env, campaign_id),
        QueryMsg::GetUpool { user_address, campaign_id } => query_user_pool(deps, env, user_address, campaign_id),
    }
}

fn query_campaign_pool(deps: Deps, _env: Env, campaign_id: String) -> StdResult<Binary> {
    let state = STATE.load(deps.storage)?;

    if let Some(campaign) = state.campaign_pool.get(&campaign_id) {
        to_binary(&campaign)
    } else {
        return Err(StdError::generic_err("Campaign does not exist"));
    }
}

fn query_user_pool(deps: Deps, _env: Env, user_address: String, reward_pool_id: String) -> StdResult<Binary> {
    let state = STATE.load(deps.storage)?;
    let user_pool_id = format!("{}_{}", user_address, reward_pool_id);

    if let Some(user_pool) = state.user_pool.get(&user_pool_id) {
        return to_binary(&user_pool)
    } else {
        return Err(StdError::generic_err("User pool does not exist"));
    }
}
