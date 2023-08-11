use crate::guess_the_number::{
    gamestate::GameState,
    routes::{init, init_with_range},
};
use axum::{routing::post, Router};
use std::{
    error::Error,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
};

mod guess_the_number;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Router::new()
        .route("/api/v1/games/guessthenumber/init", post(init))
        .with_state(GameState::new())
        .route(
            "/api/v1/games/guessthenumber/initwithrange",
            post(init_with_range),
        )
        .with_state(GameState::new());

    axum::Server::bind(&SocketAddr::V4(SocketAddrV4::new(
        Ipv4Addr::new(127, 0, 0, 1),
        3000,
    )))
    .serve(app.into_make_service())
    .await?;

    Ok(())
}
