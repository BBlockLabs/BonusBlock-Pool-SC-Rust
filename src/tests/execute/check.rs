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
fn test_check() {
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

    // successful checks on two campaigns
    let resp = check(
        deps.as_mut(),
        env.clone(),
        mock_info("creator", &[]),
        vec![
            CampaignCheckRequest {
                campaign_id: "test_campaign_1".to_string(),
                amount: Uint128::new(50),
            },
            CampaignCheckRequest {
                campaign_id: "test_campaign_2".to_string(),
                amount: Uint128::new(80),
            },
        ],
    )
    .unwrap();

    let check_responses: Vec<CampaignCheckResponse> = from_json(resp.data.unwrap()).unwrap();
    assert_eq!(
        check_responses,
        vec![
            CampaignCheckResponse {
                campaign_id: "test_campaign_1".to_string(),
                owner: "sender1".to_string(),
                amount_before_deduction: Uint128::new(100),
            },
            CampaignCheckResponse {
                campaign_id: "test_campaign_2".to_string(),
                owner: "sender2".to_string(),
                amount_before_deduction: Uint128::new(200),
            }
        ]
    );

    // verify internal state of campaign 1
    let campaign1 = CAMPAIGN_POOL
        .load(deps.as_ref().storage, "test_campaign_1".to_string())
        .unwrap();
    assert_eq!(
        campaign1,
        Campaign {
            amount: Uint128::new(50),
            owner: Addr::unchecked("sender1"),
        }
    );

    // verify internal state of campaign 2
    let campaign2 = CAMPAIGN_POOL
        .load(deps.as_ref().storage, "test_campaign_2".to_string())
        .unwrap();
    assert_eq!(
        campaign2,
        Campaign {
            amount: Uint128::new(80),
            owner: Addr::unchecked("sender2"),
        }
    );

    // verify withdrawable creation fee
    let state = STATE.load(deps.as_ref().storage).unwrap();
    assert_eq!(
        state,
        State {
            owner: deps.api.addr_canonicalize("creator").unwrap(),
            withdrawable_creation_fee: Uint128::new(170),
            claim_reward_fee: Uint128::new(999),
        }
    );
}

#[test]
fn test_check_nonexistent_campaign() {
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

    // check two campaigns, one non-existent
    let res = check(
        deps.as_mut(),
        env.clone(),
        mock_info("creator", &[]),
        vec![
            CampaignCheckRequest {
                campaign_id: "test_campaign_1".to_string(),
                amount: Uint128::new(50),
            },
            CampaignCheckRequest {
                campaign_id: "test_campaign_2".to_string(),
                amount: Uint128::new(80),
            },
        ],
    );

    assert_eq!(res, Err(StdError::generic_err("Campaign does not exist")));

    // verify internal state of campaign 1
    let campaign1 = CAMPAIGN_POOL
        .load(deps.as_ref().storage, "test_campaign_1".to_string())
        .unwrap();
    assert_eq!(
        campaign1,
        Campaign {
            amount: Uint128::new(50),
            owner: Addr::unchecked("sender1"),
        }
    );

    // verify withdrawable creation fee
    let state = STATE.load(deps.as_ref().storage).unwrap();
    assert_eq!(
        state,
        State {
            owner: deps.api.addr_canonicalize("creator").unwrap(),
            withdrawable_creation_fee: Uint128::new(50),
            claim_reward_fee: Uint128::new(999),
        }
    );
}
