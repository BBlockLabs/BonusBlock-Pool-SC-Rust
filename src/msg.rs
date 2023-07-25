use cosmwasm_std::Uint128;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InstantiateMsg {
    pub claim_reward_fee: Option<Uint128>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct UserRewardRequest {
    pub campaign_id: String,
    pub user_address: String,
    pub amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct UserRewardResponse {
    pub campaign_id: String,
    pub user_address: String,
    pub status: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CampaignCheckRequest {
    pub campaign_id: String,
    pub amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CampaignCheckResponse {
    pub campaign_id: String,
    pub owner: String,
    pub amount_before_deduction: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Deposit {
        campaign_id: String
    },
    RewardAll {
        user_rewards: Vec<UserRewardRequest>
    },
    Claim {
        campaign_id: String
    },
    Check {
        requests: Vec<CampaignCheckRequest>
    },
    Withdraw {
        amount: Uint128,
    },
    WithdrawFee {},
    SetRefundable {
        campaign_id: String
    },
    Cancel {
        campaign_id: String
    },
    SetCpool {
        campaign_id: String,
        amount: Uint128,
    },
    SetUpool {
        user_address: String,
        reward_pool_id: String,
        amount: Uint128,
    },
    SetClaimFee {
        claim_fee: Uint128
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetCpool { campaign_id: String },
    GetUpool { user_address: String, campaign_id: String },
    GetClaimFee {}
}

