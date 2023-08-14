use std::sync::Arc;

use axum::{
    extract::{Query, State},
    Json,
};
use guess_the_number::{Game, Settings};
use tokio::sync::Mutex;

use crate::guess_the_number::response::{create_response, ResponseData, ResponseStatus};

use super::{
    request::{InitCustomParams, InitRangeParams, PlayParams},
    response::{InformationResponse, InitialisationResponse, PlayResponse, Response},
    state::GameState,
};

pub async fn init(State(state): State<Arc<Mutex<GameState>>>) -> Json<Response> {
    println!("init endpoint");
    let mut game_state = &mut *state.lock().await;
    game_state.game = Some(Game::new(Settings::default()));
    let init_response = create_initialisation_response(&game_state.game.clone().unwrap());
    let response = create_response(
        ResponseStatus::SUCCESS,
        201,
        Some(ResponseData::Initialisation(init_response)),
    );
    Json::from(response)
}

pub async fn init_with_range(
    State(state): State<Arc<Mutex<GameState>>>,
    Query(range): Query<InitRangeParams>,
) -> Json<Response> {
    println!("init with range endpoint");
    let mut game_state = &mut *state.lock().await;
    game_state.game = Some(Game::new(Settings::new_with_range(range.min, range.max)));
    let init_response = create_initialisation_response(&game_state.game.clone().unwrap());
    let response = create_response(
        ResponseStatus::SUCCESS,
        201,
        Some(ResponseData::Initialisation(init_response)),
    );
    Json::from(response)
}

pub async fn init_custom(
    State(state): State<Arc<Mutex<GameState>>>,
    Query(custom): Query<InitCustomParams>,
) -> Json<Response> {
    println!("init custom endpoint");
    let mut game_state = &mut *state.lock().await;
    game_state.game = Some(Game::new(Settings::new_with_range_and_max_tries(
        custom.min,
        custom.max,
        custom.tries,
    )));
    let init_response = create_initialisation_response(&game_state.game.clone().unwrap());
    let response = create_response(
        ResponseStatus::SUCCESS,
        201,
        Some(ResponseData::Initialisation(init_response)),
    );
    Json::from(response)
}

pub async fn information(State(state): State<Arc<Mutex<GameState>>>) -> Json<Response> {
    println!("information endpoint");
    let game_state = &*state.lock().await;
    if let Some(game) = &game_state.game {
        let response = create_response(
            ResponseStatus::SUCCESS,
            200,
            Some(ResponseData::Information(create_information_response(game))),
        );
        return Json::from(response);
    }
    let response = create_response(ResponseStatus::ERROR, 404, None);
    Json::from(response)
}

pub async fn attempt(
    State(state): State<Arc<Mutex<GameState>>>,
    Query(play_params): Query<PlayParams>,
) -> Json<Response> {
    println!("attempt endpoint");
    let game_state = &mut *state.lock().await;
    if let Some(game) = &mut game_state.game {
        let play_response = {
            match game.guess(play_params.user_guess) {
                // User won
                (true, None) => {
                    PlayResponse {
                        won: true,
                        can_play_more: true, // does not matter
                        hint: None,
                    }
                }

                // User lost and/or can't play anymore
                (false, None) => PlayResponse {
                    won: false,
                    can_play_more: false,
                    hint: None,
                },

                // Incorrect guess
                (false, Some(hint)) => PlayResponse {
                    won: false,
                    can_play_more: game.can_play_more(),
                    hint: Some(hint),
                },

                // Unexpected game situation
                _ => {
                    panic!("Unepexted game situation");
                }
            }
        };
        let response = create_response(
            ResponseStatus::SUCCESS,
            200,
            Some(ResponseData::Play(play_response)),
        );
        return Json::from(response);
    }
    let response = create_response(ResponseStatus::ERROR, 404, None);
    Json::from(response)
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
