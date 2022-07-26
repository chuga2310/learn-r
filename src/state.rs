use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
/*  Pen */
use cosmwasm_std::Storage;
use cosmwasm_storage::{bucket, bucket_read, Bucket, ReadonlyBucket};

use std::str::FromStr;

static STORE_KEY: &[u8] = b"pen_storage";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Pen {
    pub id: String,
    pub owner: String,
    pub extension: ExtensionPen,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct ExtensionPen {
    pub quality: Quality,
    pub level: i32,
    pub effect: i32,
    pub resilience: i32,
    pub number_of_mints: i32,
    pub durability: i32,
}

pub fn store(storage: &mut dyn Storage) -> Bucket<Pen> {
    bucket(storage, STORE_KEY)
}

pub fn store_query(storage: &dyn Storage) -> ReadonlyBucket<Pen> {
    bucket_read(storage, STORE_KEY)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub enum Quality {
    Rare,
    Common,
    Unique,
}

impl FromStr for Quality {
    type Err = ();
    fn from_str(input: &str) -> Result<Quality, ()> {
        match input {
            "rare" => Ok(Quality::Rare),
            "common" => Ok(Quality::Common),
            "unique" => Ok(Quality::Unique),
            _ => Err(()),
        }
    }
}

// fn as_str(&self) -> &'static str {
//     match self {
//         Quality::Rare => "rare",
//         Quality::Common => "common",
//         Quality::Unique => "unique",
//     }
// }
