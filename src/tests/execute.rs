use std::vec;

use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{
    coins, from_binary, from_json, Addr, Api, BankMsg, CosmosMsg, DepsMut, Env, MessageInfo,
    StdError, SubMsg, Uint128,
};

use crate::contract::{claim, deposit, instantiate, reward_all};
use crate::msg::{InstantiateMsg, UserRewardRequest, UserRewardResponse};
use crate::state::{Campaign, State, CAMPAIGN_POOL, STATE, USER_POOL};

fn set_up(deps: DepsMut) {
    let msg: InstantiateMsg = InstantiateMsg {
        claim_reward_fee: None,
    };
    let info = mock_info("creator", &[]);
    let env = mock_env();
    instantiate(deps, env.clone(), info.clone(), msg.clone()).unwrap();
}

#[test]
fn test_deposit() {
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
        mock_info("sender", &coins(1000000, "")),
        "test_campaign_1".to_string(),
    )
    .unwrap();

    let campaign = CAMPAIGN_POOL.load(deps.as_ref().storage, "test_campaign_1".to_string());
    assert_eq!(
        campaign,
        Ok(Campaign {
            amount: Uint128::new(1000000),
            owner: Addr::unchecked("sender"),
            refundable: false,
        })
    );

    // deposit again to the same campaign_id
    deposit(
        deps.as_mut(),
        env.clone(),
        mock_info("sender2", &coins(1000000, "")),
        "test_campaign_1".to_string(),
    )
    .unwrap();

    let campaign = CAMPAIGN_POOL.load(deps.as_ref().storage, "test_campaign_1".to_string());
    assert_eq!(
        campaign,
        Ok(Campaign {
            amount: Uint128::new(2000000),
            owner: Addr::unchecked("sender"),
            refundable: false,
        })
    );
}

#[test]
fn test_reward_all() {
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
        mock_info("sender", &coins(1000000, "")),
        "test_campaign_1".to_string(),
    )
    .unwrap();

    // reward the users
    let resp = reward_all(
        deps.as_mut(),
        env.clone(),
        mock_info("creator", &[]),
        vec![
            UserRewardRequest {
                campaign_id: "test_campaign_1".to_string(),
                user_address: "user1".to_string(),
                amount: Uint128::new(1000000),
            },
            UserRewardRequest {
                campaign_id: "test_campaign_1".to_string(),
                user_address: "user2".to_string(),
                amount: Uint128::new(1000000),
            },
        ],
    )
    .unwrap();

    // assert response
    let user_responses: Vec<UserRewardResponse> = from_json(&resp.data.unwrap()).unwrap();
    assert_eq!(
        user_responses,
        vec![
            UserRewardResponse {
                campaign_id: "test_campaign_1".to_string(),
                user_address: "user1".to_string(),
                status: true,
            },
            UserRewardResponse {
                campaign_id: "test_campaign_1".to_string(),
                user_address: "user2".to_string(),
                status: false,
            },
        ]
    );

    // assert inner state of the contract
    let campaign = CAMPAIGN_POOL
        .load(deps.as_ref().storage, "test_campaign_1".to_string())
        .unwrap();
    assert_eq!(
        campaign,
        Campaign {
            amount: Uint128::new(0),
            owner: Addr::unchecked("sender"),
            refundable: false,
        }
    );

    assert_eq!(
        USER_POOL
            .load(deps.as_ref().storage, "user1_test_campaign_1".to_string())
            .unwrap(),
        Uint128::new(1000000)
    );

    assert_eq!(
        USER_POOL.has(deps.as_ref().storage, "user2_test_campaign_1".to_string()),
        false
    );
}

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
        mock_info("sender", &coins(1000000, "")),
        "test_campaign_1".to_string(),
    )
    .unwrap();

    // reward the users
    reward_all(
        deps.as_mut(),
        env.clone(),
        mock_info("creator", &[]),
        vec![
            UserRewardRequest {
                campaign_id: "test_campaign_1".to_string(),
                user_address: "user1".to_string(),
                amount: Uint128::new(1000000),
            },
            UserRewardRequest {
                campaign_id: "test_campaign_1".to_string(),
                user_address: "user2".to_string(),
                amount: Uint128::new(1000000),
            },
        ],
    )
    .unwrap();

    // try to claim from user1 without enough claim fees
    let resp = claim(
        deps.as_mut(),
        env.clone(),
        mock_info("user2", &coins(998, "")),
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
            amount: coins(1000000, ""),
        }))]
    );

    assert_eq!(
        USER_POOL.has(deps.as_ref().storage, "user1_test_campaign_1".to_string()),
        false
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
