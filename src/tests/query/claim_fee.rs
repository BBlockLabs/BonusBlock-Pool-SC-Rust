use std::vec;

use cosmwasm_std::testing::{
    mock_dependencies, mock_dependencies_with_balance, mock_env, mock_info,
};
use cosmwasm_std::{
    coins, from_binary, from_json, Addr, Api, BankMsg, CanonicalAddr, CosmosMsg, Deps, DepsMut,
    Env, MessageInfo, StdError, SubMsg, Uint128,
};

use crate::contract::{
    cancel,  claim, deposit, instantiate, query_campaign_pool, query_user_pool, reward_all,
    set_claim_fee, set_cpool, set_refundable, set_upool, withdraw,  query_claim_fee,
};
use crate::msg::{
     InstantiateMsg, UserRewardRequest,
    UserRewardResponse,
};
use crate::state::{Campaign, State, CAMPAIGN_POOL, STATE, USER_POOL};

#[test]
fn test_query_claim_fee_default_value(){
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

    let res = query_claim_fee(deps.as_ref(), env.clone()).unwrap();
    let claim_fee: Uint128 = from_json(res).unwrap();
    assert_eq!(claim_fee, Uint128::new(1000000000000000000));
}

#[test]
fn test_query_claim_fee_manual_value(){
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

    let res = query_claim_fee(deps.as_ref(), env.clone()).unwrap();
    let claim_fee: Uint128 = from_json(res).unwrap();
    assert_eq!(claim_fee, Uint128::new(999));
}