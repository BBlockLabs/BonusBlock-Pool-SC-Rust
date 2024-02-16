use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Binary, Uint128};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InstantiateMsg {
    pub pubkey: Binary,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct UserRewardRequest {
    pub campaign_id: String,
    pub user_address: String,
    pub denom: String,
    pub amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct UserRewardResponse {
    pub campaign_id: String,
    pub user_address: String,
    pub status: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    EditAdmin {
        new_admin: Addr,
    },
    Deposit {
        campaign_id: String,
    },
    Claim {
        campaign_id: String,
        amount: Uint128,
        denom: String,
        nonce: String,
        signature: Binary,
    },
    Withdraw {
        amount: Uint128,
    },
    Cancel {
        campaign_id: String,
    },
    SetCpool {
        campaign_id: String,
        amount: Uint128,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetCpool { campaign_id: String },
}

#[cw_serde]
pub struct SignedData {
    pub campaign_id: String,
    pub nonce: String,
    pub denom: String,
    pub amount: Uint128,
    pub sender: Addr,
}
