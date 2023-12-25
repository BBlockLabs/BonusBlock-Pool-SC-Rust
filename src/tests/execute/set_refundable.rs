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
fn test_set_refundable() {
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

    assert!(set_refundable(
        deps.as_mut(),
        env.clone(),
        mock_info("creator", &[]),
        "test_campaign_1".to_string(),
    )
    .is_ok());

    // check campaign
    let campaign = CAMPAIGN_POOL
        .load(deps.as_ref().storage, "test_campaign_1".to_string())
        .unwrap();
    assert_eq!(
        campaign,
        Campaign {
            amount: Uint128::new(100),
            owner: Addr::unchecked("sender1"),
            refundable: true,
        }
    );
}


#[test]
fn test_set_refundable_on_non_existent_campaign() {
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

    let res = set_refundable(
        deps.as_mut(),
        env.clone(),
        mock_info("creator", &[]),
        "test_campaign_1".to_string(),
    );
    assert_eq!(res, Err(StdError::generic_err("Campaign does not exist")));

    // check campaign
    assert_eq!(
        CAMPAIGN_POOL.has(deps.as_ref().storage, "test_campaign_1".to_string()),
        false
    );
}

#[test]
fn test_set_refundable_unauthorized() {
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


    let res = set_refundable(
        deps.as_mut(),
        env.clone(),
        mock_info("not_creator", &[]),
        "test_campaign_1".to_string(),
    );
    assert_eq!(res, Err(StdError::generic_err("Only contract owner can make the campaign refundable")));

    // check campaign
    assert_eq!(
        CAMPAIGN_POOL.load(deps.as_ref().storage, "test_campaign_1".to_string()).unwrap(),
        Campaign {
            amount: Uint128::new(100),
            owner: Addr::unchecked("sender1"),
            refundable: false,
        }
    );
}
