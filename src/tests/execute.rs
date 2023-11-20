use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{coins, Api, DepsMut, Env, MessageInfo, Uint128};

use crate::contract::{deposit, instantiate};
use crate::msg::InstantiateMsg;
use crate::state::{Campaign, State, CAMPAIGN_POOL, STATE};

fn set_up(deps: DepsMut, env: &Env, info: &MessageInfo) {
    let msg: InstantiateMsg = InstantiateMsg {
        claim_reward_fee: None,
    };
    instantiate(deps, env.clone(), info.clone(), msg.clone()).unwrap();
}

#[test]
fn test_deposit() {
    let mut deps = mock_dependencies();
    let info = mock_info("sender", &coins(1000000, ""));
    let env = mock_env();
    set_up(deps.as_mut(), &env, &info);

    deposit(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        "test_campaign_1".to_string(),
    )
    .unwrap();

    let campaign = CAMPAIGN_POOL.load(deps.as_ref().storage, "test_campaign_1".to_string());
    assert_eq!(
        campaign,
        Ok(Campaign {
            amount: Uint128::new(1000000),
            owner: info.sender,
            refundable: false,
        })
    );
}
