use std::vec;

use cosmwasm_std::testing::{
    mock_dependencies, mock_dependencies_with_balance, mock_env, mock_info,
};
use cosmwasm_std::{
    coins, from_binary, from_json, Addr, Api, BankMsg, CanonicalAddr, CosmosMsg, DepsMut, Env,
    MessageInfo, StdError, SubMsg, Uint128,
};

use crate::contract::{cancel, claim, deposit, instantiate, set_cpool, withdraw, withdraw_fee};
use crate::msg::{InstantiateMsg, UserRewardRequest, UserRewardResponse};
use crate::state::{Campaign, State, CAMPAIGN_POOL, STATE};

#[test]
fn test_withdraw_fee() {
    let mut deps = mock_dependencies();
    let env = mock_env();

    instantiate(
        deps.as_mut(),
        env.clone(),
        mock_info("creator", &[]),
        InstantiateMsg {
            claim_reward_fee: Some(Uint128::new(999)),
        },
    )
    .unwrap();

    deposit(
        deps.as_mut(),
        env.clone(),
        mock_info("sender1", &coins(100, "")),
        "test_campaign_1".to_string(),
    )
    .unwrap();

    deposit(
        deps.as_mut(),
        env.clone(),
        mock_info("sender2", &coins(200, "")),
        "test_campaign_2".to_string(),
    )
    .unwrap();

    // withdraw fee
    let resp = withdraw_fee(deps.as_mut(), env.clone(), mock_info("creator", &[])).unwrap();
    assert_eq!(
        resp.messages,
        vec![SubMsg::new(CosmosMsg::Bank(BankMsg::Send {
            to_address: "creator".to_string(),
            amount: coins(170, ""),
        }))]
    );
}

#[test]
fn test_withdraw_fee_unauthorized() {
    let mut deps = mock_dependencies();
    let env = mock_env();

    instantiate(
        deps.as_mut(),
        env.clone(),
        mock_info("creator", &[]),
        InstantiateMsg {
            claim_reward_fee: Some(Uint128::new(999)),
        },
    )
    .unwrap();

    deposit(
        deps.as_mut(),
        env.clone(),
        mock_info("sender1", &coins(100, "")),
        "test_campaign_1".to_string(),
    )
    .unwrap();

    deposit(
        deps.as_mut(),
        env.clone(),
        mock_info("sender2", &coins(200, "")),
        "test_campaign_2".to_string(),
    )
    .unwrap();

    // withdraw fee
    let res = withdraw_fee(deps.as_mut(), env.clone(), mock_info("not_creator", &[]));
    assert_eq!(
        res,
        Err(StdError::generic_err("Only contract owner can withdraw"))
    );
}
