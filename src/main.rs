#![allow(unused)]

use std::net::SocketAddr;

use axum::{Router, response::{Html, IntoResponse, Response}, routing::{get, post, get_service}, extract::{Query, Path}, middleware};
use serde::Deserialize;
use tower_http::services::ServeDir;

mod error;
mod web;

pub use self::error::{Error, Result};

#[tokio::main]
async fn main() {
    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .layer(middleware::map_response(main_response_mapper))
        .fallback_service(routes_static());

    // region:      --- Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
    // end region:  --- Start Server
}

async fn main_response_mapper(res: Response) ->  Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

    println!();

    res
}

// Static route
fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
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
// String is a body extractor, we can only
// have one body extractor per route, and
// it has to be the last argument.
async fn handler_hello3(body: String) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello3 - {body:?}", "HANDLER");

    body
}

// end region:  --- Routes Hello
