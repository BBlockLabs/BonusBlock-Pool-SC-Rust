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
    set_refundable, set_upool, withdraw, withdraw_fee,
};
use crate::msg::{
    CampaignCheckRequest, CampaignCheckResponse, InstantiateMsg, UserRewardRequest,
    UserRewardResponse,
};
use crate::state::{Campaign, State, CAMPAIGN_POOL, STATE, USER_POOL};

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
        mock_info("sender", &coins(1000, "")),
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
                amount: Uint128::new(999),
            },
            UserRewardRequest {
                campaign_id: "test_campaign_1".to_string(),
                user_address: "user2".to_string(),
                amount: Uint128::new(2),
            },
        ],
    )
    .unwrap();

    // assert response
    let user_responses: Vec<UserRewardResponse> = from_json(resp.data.unwrap()).unwrap();
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
            amount: Uint128::new(1),
            owner: Addr::unchecked("sender"),
            refundable: false,
        }
    );

    assert_eq!(
        USER_POOL
            .load(deps.as_ref().storage, "user1_test_campaign_1".to_string())
            .unwrap(),
        Uint128::new(999)
    );

    assert_eq!(
        USER_POOL.has(deps.as_ref().storage, "user2_test_campaign_1".to_string()),
        false
    );
}
