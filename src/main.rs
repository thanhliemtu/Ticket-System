#![allow(unused)]

use std::net::SocketAddr;

use axum::{Router, response::Html, routing::get};

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/hello",
        get(|| async { Html("Hello <strong>World!!!</strong>") })
    );

    // region:      --- Start Server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("->> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
    // end region:  --- Start Server
}


