use cosmwasm_std::{Addr, Binary, CanonicalAddr, Uint128};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const ADMIN: Item<CanonicalAddr> = Item::new("admin");
pub const PUBKEY: Item<Binary> = Item::new("key");

pub const NONCES: Map<&str, bool> = Map::new("nonces");
pub const CAMPAIGN_POOL: Map<String, Campaign> = Map::new("campaign_pool");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Campaign {
    pub amount: Uint128,
    pub owner: Addr,
}
