use std::vec;

use cosmwasm_std::testing::{
    mock_dependencies, mock_dependencies_with_balance, mock_env, mock_info,
};
use cosmwasm_std::{
    coins, from_binary, from_json, to_json_binary, Addr, Api, BankMsg, CanonicalAddr, CosmosMsg,
    DepsMut, Env, MessageInfo, StdError, SubMsg, Uint128,
};

use crate::contract::{cancel, claim, deposit, instantiate, set_cpool, withdraw};
use crate::msg::{InstantiateMsg, UserRewardRequest, UserRewardResponse};
use crate::state::{Campaign, CAMPAIGN_POOL};

#[test]
fn test_withdraw() {
    let mut deps = mock_dependencies_with_balance(&coins(1999, ""));
    let env = mock_env();

    instantiate(
        deps.as_mut(),
        env.clone(),
        mock_info("creator", &[]),
        InstantiateMsg {
            pubkey: to_json_binary(&"test_key".to_string()).unwrap(),
        },
    )
    .unwrap();

    deposit(
        deps.as_mut(),
        env.clone(),
        mock_info("sender", &coins(1000, "")),
        "test_campaign_1".to_string(),
    )
    .unwrap();

    // try to claim from user1
    claim(
        deps.as_mut(),
        env.clone(),
        mock_info("user1", &coins(999, "")),
        "test_campaign_1".to_string(),
        "test_token".to_string(),
        Uint128::new(999),
        "test_nonce_1".to_string(),
    )
    .unwrap();

    // try to withdraw
    let resp = withdraw(
        deps.as_mut(),
        env.clone(),
        mock_info("creator", &[]),
        Uint128::new(999),
    )
    .unwrap();

    assert_eq!(
        resp.messages,
        vec![SubMsg::new(CosmosMsg::Bank(BankMsg::Send {
            to_address: "creator".to_string(),
            amount: coins(999, ""),
        }))]
    );
}

#[test]
fn test_withdraw_unauthorized() {
    let mut deps = mock_dependencies_with_balance(&coins(1999, ""));
    let env = mock_env();

    instantiate(
        deps.as_mut(),
        env.clone(),
        mock_info("creator", &[]),
        InstantiateMsg {
            pubkey: to_json_binary(&"test_key".to_string()).unwrap(),
        },
    )
    .unwrap();

    deposit(
        deps.as_mut(),
        env.clone(),
        mock_info("sender", &coins(1000, "")),
        "test_campaign_1".to_string(),
    )
    .unwrap();

    // try to claim from user1
    claim(
        deps.as_mut(),
        env.clone(),
        mock_info("user1", &coins(999, "")),
        "test_campaign_1".to_string(),
    )
    .unwrap();

    // try to withdraw
    let res = withdraw(
        deps.as_mut(),
        env.clone(),
        mock_info("not_creator", &[]),
        Uint128::new(999),
    );
    assert_eq!(
        res,
        Err(StdError::generic_err("Only contract owner can withdraw"))
    );
}
