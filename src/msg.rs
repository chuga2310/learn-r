use crate::state::Pen;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct InstantiateMsg {
    pub id: String,
    pub owner: String,
    pub quality: String,
    pub level: i32,
    pub effect: i32,
    pub resilience:i32,
    pub number_of_mints: i32,
    pub durability: i32
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Mint {
        id: String,
        owner: String,
        quality: String,
        level: i32,
        effect: i32,
        resilience:i32,
        number_of_mints: i32,
        durability: i32
    },
    // Sell {
    //     id: String,
    //     amount: i32,
    // },
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetPen returns the pen's information
    GetPen { id: String },
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct PenInfoResponse {
    pub pen: Option<Pen>,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct CountResponse {
    pub count: i32,
}
