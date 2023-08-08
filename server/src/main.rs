use axum::{
    extract::{FromRef, Path, State},
    routing::{get, post},
    Json, Router,
};
use guess_the_number::GuessTheNumber;
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
};

#[derive(Serialize, Deserialize)]
struct GameResponse {
    hint: String,
}

async fn guess_the_number(
    Path(user_guess): Path<i32>,
    State(mut game): State<GuessTheNumber>,
) -> Json<GameResponse> {
    let hint = {
        if game.guess(user_guess) {
            String::from("WON")
        } else {
            String::from("TRY AGAIN")
        }
    };
    GameResponse { hint }.into()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let game = GuessTheNumber::new(10, 1, 20);

    let app = Router::new()
        .route("/games/guessthenumber/:user_guess", post(guess_the_number))
        .with_state(game);

    axum::Server::bind(&SocketAddr::V4(SocketAddrV4::new(
        Ipv4Addr::new(127, 0, 0, 1),
        3000,
    )))
    .serve(app.into_make_service())
    .await?;

    Ok(())
}
