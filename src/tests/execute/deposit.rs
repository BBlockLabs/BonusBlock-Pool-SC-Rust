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
fn test_deposit() {
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

    // deposit to create a campaign
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
        })
    );

    // deposit again to the same campaign_id as different sender
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
        })
    );
}
