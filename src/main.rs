#![allow(unused)]

use std::net::SocketAddr;

use axum::{Router, response::{Html, IntoResponse}, routing::{get, post}, extract::{Query, Path}};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().merge(routes_hello());
        
    // region:      --- Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
    // end region:  --- Start Server
}

// region:      --- Routes Hello
fn routes_hello() -> Router {
    Router::new()
    .route("/hello", get(handler_hello))
    .route("/hello2/:name", get(handler_hello2))
    .route("/hello3", post(handler_hello3))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// e.g., `/hello?name=Haruka`
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");
    
    let name = params.name.as_deref().unwrap_or("World!");

    Html(format!("Hello <strong>{name}!/strong>"))
}

// e.g., `/hello2/Hansamu`
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello2 - {name:?}", "HANDLER");

    Html(format!("Hello <strong>{name}!/strong>"))
}


// post request handler, returns the body
async fn handler_hello3(body: String) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello3 - {body:?}", "HANDLER");

    body
}

// end region:  --- Routes Hello
