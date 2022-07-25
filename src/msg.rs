use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::state::Pen;


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub id: String,
    pub owner: String,
    pub quality: String,
    pub level: Number,
    pub effect: Number,
    pub resilience:Number,
    pub number_of_mints: Number,
    pub durability: Number
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Mint {
        id: String,
        pub owner: String,
        pub quality: String,
        pub level: Number,
        pub effect: Number,
        pub resilience:Number,
        pub number_of_mints: Number,
        pub durability: Number
    },
    // Sell {
    //     id: String,
    //     amount: i32,
    // },
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetPen returns the pen's information
    GetPen { id: String },
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PenInfoResponse {
    pub pen: Option<Pen>,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CountResponse {
    pub count: i32,
}
