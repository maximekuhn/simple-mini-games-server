use std::sync::Arc;

use axum::{
    routing::{get, post},
    Extension, Router,
};
use tokio::sync::Mutex;

use self::{
    routes::{information, init},
    state::GameState,
};

mod request;
mod response;
mod routes;
mod state;

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
        .route("/init", post(init).with_state(game_state.clone()))
        .route(
            "/information",
            get(information).with_state(game_state.clone()),
        );

    // Return the upgraded router
    router
}
