use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{Api, Uint128};

use crate::contract::instantiate;
use crate::msg::InstantiateMsg;
use crate::state::{ADMIN, PUBKEY};

#[test]
fn test_instantiate_default() {
    let mut deps = mock_dependencies();
    let info = mock_info("sender", &[]);
    let env = mock_env();
    let msg = InstantiateMsg {
        claim_reward_fee: None,
    };

    instantiate(deps.as_mut(), env.clone(), info.clone(), msg.clone()).unwrap();

    let state = STATE.load(deps.as_ref().storage).unwrap();
    assert_eq!(
        state,
        State {
            owner: deps.api.addr_canonicalize("sender").unwrap(),
            withdrawable_creation_fee: Uint128::zero(),
        }
    );
}

#[test]
fn test_instantiate_with_custom_claim_fee() {
    let mut deps = mock_dependencies();
    let info = mock_info("sender", &[]);
    let env = mock_env();
    let msg = InstantiateMsg {
        claim_reward_fee: Some(Uint128::new(99)),
    };

    instantiate(deps.as_mut(), env.clone(), info.clone(), msg.clone()).unwrap();

    let state = STATE.load(deps.as_ref().storage).unwrap();
    assert_eq!(
        state,
        State {
            owner: deps.api.addr_canonicalize("sender").unwrap(),
            withdrawable_creation_fee: Uint128::zero(),
        }
    );
}
