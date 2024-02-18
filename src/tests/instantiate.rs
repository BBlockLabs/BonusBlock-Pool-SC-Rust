use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{to_json_binary, Api};

use crate::contract::instantiate;
use crate::msg::InstantiateMsg;
use crate::state::{ADMIN, PUBKEY};

#[test]
fn test_instantiate_default() {
    let mut deps = mock_dependencies();
    let info = mock_info("sender", &[]);
    let env = mock_env();
    let msg = InstantiateMsg {
        pubkey: to_json_binary(&"test_key".to_string()).unwrap(),
    };

    instantiate(deps.as_mut(), env.clone(), info.clone(), msg.clone()).unwrap();

    let admin = ADMIN.load(deps.as_ref().storage).unwrap();
    assert_eq!(admin, deps.api.addr_canonicalize("sender").unwrap());
}

#[test]
fn test_instantiate_with_custom_claim_fee() {
    let mut deps = mock_dependencies();
    let info = mock_info("sender", &[]);
    let env = mock_env();
    let msg = InstantiateMsg {
        pubkey: to_json_binary(&"test_key".to_string()).unwrap(),
    };

    instantiate(deps.as_mut(), env.clone(), info.clone(), msg.clone()).unwrap();

    let admin = ADMIN.load(deps.as_ref().storage).unwrap();
    let pubkey = PUBKEY.load(&deps.storage).unwrap();
    assert_eq!(admin, deps.api.addr_canonicalize("sender").unwrap());
}
