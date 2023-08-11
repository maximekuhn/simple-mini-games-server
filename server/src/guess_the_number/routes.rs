use axum::{
    extract::{Query, State},
    Json,
};
use guess_the_number::{Game, Settings};
use serde::{Deserialize, Serialize};

use super::{
    gamestate::GameState,
    response::{Data, Init, Response},
};

/// Initialize a new game
/// Route: /api/v1/games/guessthenumber/init
/// HTTP method: POST
pub async fn init(State(mut game): State<GameState>) -> Json<Response> {
    // Initialize the game with default settings
    game.game = Some(Game::new(Settings::default()));

    // Create response
    let (_, game_settings) = game.game.unwrap().information();
    let response = Response {
        status: String::from("success"),
        code: 201,
        data: Data::INIT(Init {
            min_number: game_settings.min_number,
            max_number: game_settings.max_number,
            max_tries: game_settings.max_tries,
        }),
    };

    // Send response
    response.into()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Param {
    pub min: i32,
    pub max: i32,
}

/// Initialize a new game with specified range
/// Route: /api/v1/games/guessthenumber/init-with-range
/// HTTP method: POST
pub async fn init_with_range(
    Query(params): Query<Param>,
    State(mut game): State<GameState>,
) -> Json<Response> {
    // Initialize a new game with specified range
    let settings = Settings::new_with_range(params.min, params.max);
    game.game = Some(Game::new(settings));

    // Create response
    let (_, game_settings) = game.game.unwrap().information();
    let response = Response {
        status: String::from("success"),
        code: 201,
        data: Data::INIT(Init {
            min_number: game_settings.min_number,
            max_number: game_settings.max_number,
            max_tries: game_settings.max_tries,
        }),
    };

    // Send response
    response.into()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamCustom {
    pub min: i32,
    pub max: i32,
    pub tries: u8,
}

/// Initialize a new game with specified range and maximum amount of tries
/// Route: /api/v1/games/guessthenumber/init-custom
/// HTTP method: POST
pub async fn init_custom(
    Query(params): Query<ParamCustom>,
    State(mut game): State<GameState>,
) -> Json<Response> {
    // Initialize a new game with specified range
    let settings = Settings::new_with_range_and_max_tries(params.min, params.max, params.tries);
    game.game = Some(Game::new(settings));

    // Create response
    let (_, game_settings) = game.game.unwrap().information();
    let response = Response {
        status: String::from("success"),
        code: 201,
        data: Data::INIT(Init {
            min_number: game_settings.min_number,
            max_number: game_settings.max_number,
            max_tries: game_settings.max_tries,
        }),
    };

    // Send response
    response.into()
}
