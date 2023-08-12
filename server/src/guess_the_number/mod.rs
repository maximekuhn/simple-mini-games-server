use std::sync::Arc;

use axum::{
    routing::{get, post},
    Extension, Router,
};
use tokio::sync::Mutex;

use self::{
    routes::{information, init, init_custom, init_with_range},
    state::GameState,
};

mod request;
mod response;
mod routes;
mod state;

const PREFIX: &str = "/api/v1/games/guessthenumber";

/// Add all routes related to guess the number game and
/// returns the router.
///
/// # Arguments
/// * `router` : axum router used to create the server
pub fn add_games_routes(mut router: Router) -> Router {
    // Hold the game object
    let game_state = Arc::new(Mutex::new(GameState::new()));

    // Add all required routes
    router = router
        .route(
            format!("{}/init", PREFIX).as_str(),
            post(init).with_state(game_state.clone()),
        )
        .route(
            format!("{}/init-with-range", PREFIX).as_str(),
            post(init_with_range).with_state(game_state.clone()),
        )
        .route(
            format!("{}/init-custom", PREFIX).as_str(),
            post(init_custom).with_state(game_state.clone()),
        )
        .route(
            format!("{}/information", PREFIX).as_str(),
            get(information).with_state(game_state.clone()),
        );

    // Return the upgraded router
    router
}
