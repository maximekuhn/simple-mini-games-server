use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct InitialisationResponse {
    pub min_number: i32,
    pub max_number: i32,
    pub max_tries: u8,
}

#[derive(Serialize, Deserialize)]
pub struct InformationResponse {
    pub min_number: i32,
    pub max_number: i32,
    pub max_tries: u8,
    pub current_tries: u8,
}

#[derive(Serialize, Deserialize)]
pub struct PlayResponse {
    pub won: bool,
    pub can_play_more: bool,
    pub hint: Option<String>,
}

/// All possible responses
#[derive(Serialize, Deserialize)]
pub enum ResponseData {
    Initialisation(InitialisationResponse),
    Information(InformationResponse),
    Play(PlayResponse),
}

/// All possible status
#[derive(Serialize, Deserialize)]
pub enum ResponseStatus {
    SUCCESS,
    ERROR,
}

/// Response
#[derive(Serialize, Deserialize)]
pub struct Response {
    pub status: ResponseStatus,
    pub code: u16,
    pub data: Option<ResponseData>,
}

pub fn create_response(status: ResponseStatus, code: u16, data: Option<ResponseData>) -> Response {
    let response = Response { status, code, data };
    response
}
