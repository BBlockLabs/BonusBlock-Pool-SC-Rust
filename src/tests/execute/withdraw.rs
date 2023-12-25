use std::vec;

use cosmwasm_std::testing::{
    mock_dependencies, mock_dependencies_with_balance, mock_env, mock_info,
};
use cosmwasm_std::{
    coins, from_binary, from_json, Addr, Api, BankMsg, CanonicalAddr, CosmosMsg, DepsMut, Env,
    MessageInfo, StdError, SubMsg, Uint128,
};

use crate::contract::{
    cancel, check, claim, deposit, instantiate, reward_all, set_claim_fee, set_cpool,
     set_upool, withdraw, withdraw_fee,
};
use crate::msg::{
    CampaignCheckRequest, CampaignCheckResponse, InstantiateMsg, UserRewardRequest,
    UserRewardResponse,
};
use crate::state::{Campaign, State, CAMPAIGN_POOL, STATE, USER_POOL};

#[test]
fn test_withdraw() {
    let mut deps = mock_dependencies_with_balance(&coins(1999, ""));
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

    // reward user1
    reward_all(
        deps.as_mut(),
        env.clone(),
        mock_info("creator", &[]),
        vec![UserRewardRequest {
            campaign_id: "test_campaign_1".to_string(),
            user_address: "user1".to_string(),
            amount: Uint128::new(1000),
        }],
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

    // reward user1
    reward_all(
        deps.as_mut(),
        env.clone(),
        mock_info("creator", &[]),
        vec![UserRewardRequest {
            campaign_id: "test_campaign_1".to_string(),
            user_address: "user1".to_string(),
            amount: Uint128::new(1000),
        }],
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
