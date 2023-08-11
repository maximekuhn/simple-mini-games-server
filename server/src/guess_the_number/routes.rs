use std::sync::Arc;

use axum::{
    extract::{Query, State},
    Extension, Json,
};
use guess_the_number::{Game, Settings};
use tokio::sync::Mutex;

use crate::guess_the_number::response::{create_response, ResponseData, ResponseStatus};

use super::{
    request::{InitCustomParams, InitRangeParams},
    response::{InformationResponse, InitialisationResponse, Response},
    state::GameState,
};

pub async fn init(State(mut state): State<Arc<Mutex<GameState>>>) -> Json<Response> {
    println!("init endpoint");
    todo!()
}

pub async fn init_with_range(
    Extension(mut state): Extension<GameState>,
    Query(range): Query<InitRangeParams>,
) -> Json<Response> {
    println!("init with range endpoint");
    todo!()
}

pub async fn init_custom(
    State(mut state): State<Arc<GameState>>,
    Query(custom): Query<InitCustomParams>,
) -> Json<Response> {
    println!("init custom endpoint");
    todo!()
}

pub async fn information(State(state): State<Arc<GameState>>) -> Json<Response> {
    println!("information endpoint");
    todo!()
}

fn create_initialisation_response(game: &Game) -> InitialisationResponse {
    let (_, settings) = game.information();
    let response = InitialisationResponse {
        min_number: settings.min_number,
        max_number: settings.max_number,
        max_tries: settings.max_tries,
    };
    response
}

fn create_information_response(game: &Game) -> InformationResponse {
    let (current_tries, settings) = game.information();
    let response = InformationResponse {
        min_number: settings.min_number,
        max_number: settings.max_number,
        max_tries: settings.max_tries,
        current_tries,
    };
    response
}
