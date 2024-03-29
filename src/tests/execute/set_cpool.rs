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
fn test_set_new_cpool() {
    let mut deps = mock_dependencies();
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

    let res = set_cpool(
        deps.as_mut(),
        env.clone(),
        mock_info("creator", &coins(100, "")),
        "test_campaign_1".to_string(),
        Uint128::new(100),
    );
    assert!(res.is_ok());

    // check campaign
    let campaign = CAMPAIGN_POOL
        .load(deps.as_ref().storage, "test_campaign_1".to_string())
        .unwrap();
    assert_eq!(
        campaign,
        Campaign {
            amount: Uint128::new(100),
            owner: Addr::unchecked("creator"),
        }
    );
}

#[test]
fn test_set_existing_cpool() {
    let mut deps = mock_dependencies();
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
        mock_info("sender1", &coins(1000, "")),
        "test_campaign_1".to_string(),
    )
    .unwrap();

    let res = set_cpool(
        deps.as_mut(),
        env.clone(),
        mock_info("creator", &coins(1000, "")),
        "test_campaign_1".to_string(),
        Uint128::new(2000),
    );
    assert!(res.is_ok());

    // check campaign
    let campaign = CAMPAIGN_POOL
        .load(deps.as_ref().storage, "test_campaign_1".to_string())
        .unwrap();
    assert_eq!(
        campaign,
        Campaign {
            amount: Uint128::new(2000),
            owner: Addr::unchecked("sender1"),
        }
    );
}

#[test]
fn test_set_new_cpool_unauthorized() {
    let mut deps = mock_dependencies();
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

    let res = set_cpool(
        deps.as_mut(),
        env.clone(),
        mock_info("not_creator", &coins(100, "")),
        "test_campaign_1".to_string(),
        Uint128::new(100),
    );
    assert_eq!(
        res.unwrap_err(),
        StdError::generic_err("Only contract owner can set the campaign pool")
    );
}
