use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
/*  Pen */
use cosmwasm_std::Storage;
use cosmwasm_storage::{bucket, bucket_read, Bucket, ReadonlyBucket};

static STORE_KEY: &[u8] = b"pen_storage";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Pen {
    pub id: String,
    pub owner: String,
}

pub fn store(storage: &mut dyn Storage) -> Bucket<Pen> {
    bucket(storage, STORE_KEY)
}

pub fn store_query(storage: &dyn Storage) -> ReadonlyBucket<Pen> {
    bucket_read(storage, STORE_KEY)
}
