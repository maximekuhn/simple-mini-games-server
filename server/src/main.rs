use crate::guess_the_number::add_games_routes;
use axum::Router;
use std::{
    error::Error,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
};

mod guess_the_number;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Creates router
    let mut router = Router::new();

    // Add guess the number game's routes
    router = add_games_routes(router);

    // Creates server using router and start serving at 127.0.0.1:3000
    axum::Server::bind(&SocketAddr::V4(SocketAddrV4::new(
        Ipv4Addr::new(127, 0, 0, 1),
        3000,
    )))
    .serve(router.into_make_service())
    .await?;

    Ok(())
}
