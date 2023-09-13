use cosmwasm_std::{Addr, CanonicalAddr, Uint128};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Campaign {
    pub amount: Uint128,
    pub owner: Addr,
    pub refundable: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub owner: CanonicalAddr,
    pub withdrawable_creation_fee: Uint128,
    pub claim_reward_fee: Uint128,
}

pub const STATE: Item<State> = Item::new("state");
pub const CAMPAIGN_POOL: Map<String, Campaign> = Map::new("cpool");
pub const USER_POOL: Map<String, Uint128> = Map::new("upool");
