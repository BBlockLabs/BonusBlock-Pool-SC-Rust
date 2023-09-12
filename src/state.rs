use cosmwasm_std::{Addr, CanonicalAddr, Uint128};
use cw_storage_plus::{Item, Map};
use std::collections::HashMap;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Campaign {
    pub amount: Uint128,
    pub owner: Addr,
    pub refundable: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UserPool {
    pub amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub owner: CanonicalAddr,
    pub campaign_pool: HashMap<String, Campaign>,
    pub user_pool: HashMap<String, UserPool>,
    pub withdrawable_creation_fee: Uint128,
    pub claim_reward_fee: Uint128,
}

pub const STATE: Item<State> = Item::new("state");
pub const CAMPAIGN_POOL: Map<String, Campaign> = Map::new("campaign_pool");
pub const USER_POOL: Map<String, Uint128> = Map::new("user_pool");
