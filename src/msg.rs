use crate::state::{ExtensionPen, Pen};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct InstantiateMsg {
    pub id: String,
    pub owner: String,
    pub extension: ExtensionPen,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Mint {
        id: String,
        owner: String,
        extension: ExtensionPen,
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

pub enum MetadataMsg {
    GetMetadata { id: String },
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct PenInfoResponse {
    pub pen: Option<Pen>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct MetadataPenResponse {
    pub url: String,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct CountResponse {
    pub count: i32,
}
