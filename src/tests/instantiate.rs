use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{Api, Uint128};

use crate::contract::instantiate;
use crate::msg::InstantiateMsg;
use crate::state::{State, STATE};

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
            claim_reward_fee: Uint128::new(1000000000000000000),
        }
    );
}

#[test]
fn test_instantiate_with_custom_claim_fee() {
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
            claim_reward_fee: Uint128::new(1000000000000000000),
        }
    );
}
