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
    let response = Response {
        status: String::from("success"),
        code: 201,
        data: Data::INIT(Init {}),
    };

    // Send response
    response.into()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Param {
    pub min: i32,
    pub max: i32,
}

/// Initialize a new game with specified settings
/// Route: /api/v1/games/guessthenumber/init
/// HTTP method: POST
pub async fn init_with_range(
    Query(params): Query<Param>,
    State(mut game): State<GameState>,
) -> Json<Response> {
    println!("HERE");
    println!("{:?}", params);
    todo!()
}
