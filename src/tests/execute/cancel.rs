use std::vec;

use cosmwasm_std::testing::{
    mock_dependencies, mock_dependencies_with_balance, mock_env, mock_info,
};
use cosmwasm_std::{
    coins, from_binary, from_json, Addr, Api, BankMsg, CanonicalAddr, CosmosMsg, DepsMut, Env,
    MessageInfo, StdError, SubMsg, Uint128,
};

use crate::contract::{
    cancel,  claim, deposit, instantiate, reward_all, set_claim_fee, set_cpool,
    set_refundable, set_upool, withdraw, 
};
use crate::msg::{
     InstantiateMsg, UserRewardRequest,
    UserRewardResponse,
};
use crate::state::{Campaign, State, CAMPAIGN_POOL, STATE, USER_POOL};

#[test]
fn test_cancel_as_contract_owner() {
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

    set_refundable(
        deps.as_mut(),
        env.clone(),
        mock_info("creator", &[]),
        "test_campaign_1".to_string(),
    )
    .unwrap();

    let resp = cancel(
        deps.as_mut(),
        env.clone(),
        mock_info("creator", &[]),
        "test_campaign_1".to_string(),
    )
    .unwrap();

    assert_eq!(
        resp.messages,
        vec![SubMsg::new(CosmosMsg::Bank(BankMsg::Send {
            to_address: "sender1".to_string(),
            amount: coins(100, ""),
        }))]
    );

    assert_eq!(
        CAMPAIGN_POOL.has(deps.as_ref().storage, "test_campaign_1".to_string()),
        false
    );
}

#[test]
fn test_cancel_as_campaign_owner() {
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

    set_refundable(
        deps.as_mut(),
        env.clone(),
        mock_info("creator", &[]),
        "test_campaign_1".to_string(),
    )
    .unwrap();

    let resp = cancel(
        deps.as_mut(),
        env.clone(),
        mock_info("sender1", &[]),
        "test_campaign_1".to_string(),
    )
    .unwrap();

    assert_eq!(
        resp.messages,
        vec![SubMsg::new(CosmosMsg::Bank(BankMsg::Send {
            to_address: "sender1".to_string(),
            amount: coins(100, ""),
        }))]
    );

    assert_eq!(
        CAMPAIGN_POOL.has(deps.as_ref().storage, "test_campaign_1".to_string()),
        false
    );
}

#[test]
fn test_cancel_non_refundable_campaign() {
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

    let res = cancel(
        deps.as_mut(),
        env.clone(),
        mock_info("creator", &[]),
        "test_campaign_1".to_string(),
    );

    assert_eq!(
        res,
        Err(StdError::generic_err(
            "Campaign was not set to be refundable"
        ))
    );

    assert!(CAMPAIGN_POOL.has(deps.as_ref().storage, "test_campaign_1".to_string()));
}

#[test]
fn test_cancel_non_existent_campaign() {
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

    let res = cancel(
        deps.as_mut(),
        env.clone(),
        mock_info("creator", &[]),
        "test_campaign_1".to_string(),
    );

    assert_eq!(res, Err(StdError::generic_err("Campaign does not exist")));

    assert_eq!(
        CAMPAIGN_POOL.has(deps.as_ref().storage, "test_campaign_1".to_string()),
        false
    );
}
