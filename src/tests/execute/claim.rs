use std::vec;

use cosmwasm_std::testing::{
    mock_dependencies, mock_dependencies_with_balance, mock_env, mock_info,
};
use cosmwasm_std::{
    coins, from_binary, from_json, Addr, Api, BankMsg, CanonicalAddr, CosmosMsg, DepsMut, Env,
    MessageInfo, StdError, SubMsg, Uint128,
};

use crate::contract::{cancel, claim, deposit, instantiate, set_cpool, withdraw};
use crate::msg::{InstantiateMsg, UserRewardRequest, UserRewardResponse};
use crate::state::{Campaign, CAMPAIGN_POOL};

#[test]
fn test_claim() {
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
        mock_info("sender", &coins(1000, "")),
        "test_campaign_1".to_string(),
    )
    .unwrap();

    // try to claim from user1 without enough claim fees
    let resp = claim(
        deps.as_mut(),
        env.clone(),
        mock_info("user1", &coins(998, "")),
        "test_campaign_1".to_string(),
    );

    assert_eq!(
        resp,
        Err(StdError::generic_err("You must attach 999 to claim reward"))
    );

    // try to claim from user1
    let resp = claim(
        deps.as_mut(),
        env.clone(),
        mock_info("user1", &coins(999, "")),
        "test_campaign_1".to_string(),
    )
    .unwrap();

    assert_eq!(
        resp.messages,
        vec![SubMsg::new(CosmosMsg::Bank(BankMsg::Send {
            to_address: "user1".to_string(),
            amount: coins(1000, ""),
        }))]
    );

    // try to claim from user2 who doesn't exist
    let resp = claim(
        deps.as_mut(),
        env.clone(),
        mock_info("user2", &coins(999, "")),
        "test_campaign_1".to_string(),
    );

    assert_eq!(resp, Err(StdError::generic_err("User pool does not exist")));
}

#[test]
fn test_claim_zero_fee() {
    let mut deps = mock_dependencies();
    let env = mock_env();

    instantiate(
        deps.as_mut(),
        env.clone(),
        mock_info("creator", &[]),
        InstantiateMsg {
            claim_reward_fee: Some(Uint128::new(0)),
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
    let resp = claim(
        deps.as_mut(),
        env.clone(),
        mock_info("user1", &[]),
        "test_campaign_1".to_string(),
    )
    .unwrap();

    assert_eq!(
        resp.messages,
        vec![SubMsg::new(CosmosMsg::Bank(BankMsg::Send {
            to_address: "user1".to_string(),
            amount: coins(1000, ""),
        }))]
    );
}
