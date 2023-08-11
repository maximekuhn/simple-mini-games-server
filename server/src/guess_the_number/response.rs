use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Init {}

#[derive(Serialize, Deserialize)]
pub struct Information {}

#[derive(Serialize, Deserialize)]
pub struct Play {}

#[derive(Serialize, Deserialize)]
pub enum Data {
    EMPTY,
    INIT(Init),
    INFORMATION(Information),
    PLAY(Play),
    RESTART(String),
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub status: String,
    pub code: u16,
    pub data: Data,
}
