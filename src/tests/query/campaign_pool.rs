use std::vec;

use cosmwasm_std::testing::{
    mock_dependencies, mock_dependencies_with_balance, mock_env, mock_info,
};
use cosmwasm_std::{
    coins, from_binary, from_json, Addr, Api, BankMsg, CanonicalAddr, CosmosMsg, Deps, DepsMut,
    Env, MessageInfo, StdError, SubMsg, Uint128,
};

use crate::contract::{
    cancel,  claim, deposit, instantiate, query_campaign_pool, reward_all, set_claim_fee,
    set_cpool, set_refundable, set_upool, withdraw, 
};
use crate::msg::{
     InstantiateMsg, UserRewardRequest,
    UserRewardResponse,
};
use crate::state::{Campaign, State, CAMPAIGN_POOL, STATE, USER_POOL};

#[test]
fn test_query_campaign_pool() {
    let mut deps = mock_dependencies();
    let env = mock_env();

    instantiate(
        deps.as_mut(),
        env.clone(),
        mock_info("creator", &[]),
        InstantiateMsg {
            claim_reward_fee: None,
        },
    )
    .unwrap();

    deposit(
        deps.as_mut(),
        env.clone(),
        mock_info("sender1", &coins(1000, "")),
        "test_campaign_1".to_string(),
    )
    .unwrap();

    let res =
        query_campaign_pool(deps.as_ref(), env.clone(), "test_campaign_1".to_string()).unwrap();
    let campaign: Campaign = from_json(res).unwrap();
    assert_eq!(
        campaign,
        Campaign {
            amount: Uint128::new(1000),
            owner: Addr::unchecked("sender1"),
            refundable: false,
        }
    );
}

#[test]
fn test_query_campaign_pool_empty() {
    let mut deps = mock_dependencies();
    let env = mock_env();

    instantiate(
        deps.as_mut(),
        env.clone(),
        mock_info("creator", &[]),
        InstantiateMsg {
            claim_reward_fee: None,
        },
    )
    .unwrap();

    let res = query_campaign_pool(deps.as_ref(), env.clone(), "test_campaign_1".to_string());
    assert_eq!(res, Err(StdError::generic_err("Campaign does not exist")));
}
