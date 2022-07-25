use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
/*  Pen */
use cosmwasm_storage::{bucket, bucket_read, Bucket, ReadonlyBucket};
use cosmwasm_std::Storage;

static STORE_KEY: &[u8] = b"pen_storage";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Pen {
    pub id: String,
    pub owner: String,
    pub quality: String,
    pub level: Number,
    pub effect: Number,
    pub resilience:Number,
    pub number_of_mints: Number,
    pub durability: Number
}

pub fn store(storage: &mut dyn Storage) -> Bucket<Pen> {
    bucket(storage, STORE_KEY)
}

pub fn store_query(storage: &dyn Storage) -> ReadonlyBucket<Pen> {
    bucket_read(storage, STORE_KEY)
}