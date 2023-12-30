use axum::{Json, routing::post, Router};
use serde::Deserialize;
use serde_json::{Value, json};
use tower_cookies::{Cookies, Cookie};

use crate::{Error, Result, web};

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}

// Returns the router of the whole module
pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))   
}

// Json is a body extractor, and we can only have
// one body extractor per route.
// The body extractor has to be the last argument.
// We can use the Result because our custom Error
// implements IntoResponse.
// Remember the return type for a handler is impl IntoResponse
async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    // TODO: Implement real db/auth logic
    if payload.username != "demo1" || payload.pwd != "welcome" {
        return Err(Error::LoginFail);
    }
    
    // FIXME: Implement real auth-token generation/signature.
    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));

    // Create the success body
    let body = Json(json!({
        "result": { 
            "success": true 
        }
    }));

    Ok(body)
}