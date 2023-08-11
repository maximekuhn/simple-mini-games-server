use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct InitRangeParams {
    pub min: i32,
    pub max: i32,
}

#[derive(Serialize, Deserialize)]
pub struct InitCustomParams {
    pub min: i32,
    pub max: i32,
    pub tries: u8,
}
