use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
/*  Pen */
use cosmwasm_storage::{bucket, bucket_read, Bucket, ReadonlyBucket};
use cosmwasm_std::Storage;

static STORE_KEY: &[u8] = b"pen_storage";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Pen {
    pub id: String,
    pub name: String,
    pub amount: i32,
    pub price: i32,
}

pub fn store(storage: &mut dyn Storage) -> Bucket<Pen> {
    bucket(storage, STORE_KEY)
}

pub fn store_query(storage: &dyn Storage) -> ReadonlyBucket<Pen> {
    bucket_read(storage, STORE_KEY)
}