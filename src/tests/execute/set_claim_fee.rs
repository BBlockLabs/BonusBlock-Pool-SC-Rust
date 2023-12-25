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
fn test_set_claim_fee() {
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

    let res = set_claim_fee(
        deps.as_mut(),
        env.clone(),
        mock_info("creator", &[]),
        Uint128::new(456),
    );
    assert!(res.is_ok());

    // check state
    let state = STATE.load(deps.as_ref().storage).unwrap();
    assert_eq!(
        state,
        State {
            owner: deps.api.addr_canonicalize("creator").unwrap(),
            claim_reward_fee: Uint128::new(456),
        }
    );
}

#[test]
fn test_set_claim_fee_unauthorized() {
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

    let res = set_claim_fee(
        deps.as_mut(),
        env.clone(),
        mock_info("not_creator", &[]),
        Uint128::new(456),
    );
    assert_eq!(
        res,
        Err(StdError::generic_err(
            "Only contract owner can edit the claim fee"
        ))
    );

    // check state
    let state = STATE.load(deps.as_ref().storage).unwrap();
    assert_eq!(
        state,
        State {
            owner: deps.api.addr_canonicalize("creator").unwrap(),
            claim_reward_fee: Uint128::new(999),
        }
    );
}
