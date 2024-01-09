use async_trait::async_trait;
use axum::RequestPartsExt;
use axum::body::Body;
use axum::extract::{FromRequestParts, State};
use axum::http::Request;
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::response::Response;
use lazy_regex::regex_captures;
use tower_cookies::{Cookies, Cookie};

use crate::ctx::Ctx;
use crate::model::ModelController;
use crate::web::AUTH_TOKEN;
use crate::error::{Error, Result};

// We are using Result<Ctx>. If the extraction was successful
// we'll get a Ctx struct in ctx. If not, we'll get an Error.
// If we use "ctx: Ctx" instead, this function will not be called
// if extraction fails, the response will come from the extractor
pub async fn mw_require_auth(
    ctx: Result<Ctx>,
    req: Request<Body>, 
    next: Next
) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth - {ctx:?}", "MIDDLEWARE");

    ctx?; // This will handle the Ok or Err variant

    Ok(next.run(req).await)
}

pub async fn mw_ctx_resolver(
    State(_mc): State<ModelController>,
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next
) -> Result<Response> {
    println!("->> {:<12} - mw_ctx_resolver", "MIDDLEWARE");

    // Convert to an Option<String> if we have Option<Cookie>
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    // Compute Result<Ctx>.
    let result_ctx = match auth_token 
        .ok_or_else(|| Error::AuthFailNoAuthTokenCookie)
        .and_then(parse_token)
    {
        Ok((user_id, exp, sign)) => {
            // TODO: Token components validation.
            Ok(Ctx::new(user_id))  
        }
        Err(e) => Err(e)
    };

    // Remove the cookie if something went wrong other than NoAuthTokenCookie.
    if result_ctx.is_err()
        && !matches!(result_ctx, Err(Error::AuthFailNoAuthTokenCookie)) 
    {
        cookies.remove(Cookie::from(AUTH_TOKEN))
    }

    // Store the ctx_result in the request extension
    req.extensions_mut().insert(result_ctx);

    Ok(next.run(req).await)
}

// region:      --- Ctx Extractor
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        println!("->> {:<12} - Ctx", "EXTRACTOR");
        
        parts
        .extensions
        .get::<Result<Ctx>>()
        .ok_or_else(|| Error::AuthFailCtxNotInRequestExt)?
        .clone()
    }
}

// end region:  --- Ctx Extractor

// Parse a token of format `user-[user-id].[expiration].[signature]`
// Returns (user_id, expiration, signature)
fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sign) = regex_captures!(
        r#"^user-(\d+)\.(.+)\.(.+)"#, // a literal regex
        &token
    )
    .ok_or_else(|| Error::AuthFailTokenWrongFormat)?;

    let user_id: u64 = user_id
    .parse()
    .map_err(|_| Error::AuthFailTokenWrongFormat)?;
    // .or_else(|_| Err(Error::AuthFailTokenWrongFormat))?;

    Ok((user_id, exp.to_string(), sign.to_string()))
}