use axum::{routing::get, Router};
use std::{
    error::Error,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Router::new().route("/", get(|| async { "Hello, world" }));

    axum::Server::bind(&SocketAddr::V4(SocketAddrV4::new(
        Ipv4Addr::new(127, 0, 0, 1),
        3000,
    )))
    .serve(app.into_make_service())
    .await?;

    Ok(())
}
