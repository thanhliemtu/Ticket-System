use async_trait::async_trait;
use axum::RequestPartsExt;
use axum::extract::FromRequestParts;
use axum::http::Request;
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::response::Response;
use lazy_regex::regex_captures;
use tower_cookies::Cookies;

use crate::ctx::Ctx;
use crate::web::AUTH_TOKEN;
use crate::error::{Error, Result};

// We are using Result<Ctx>. If the extraction was successful
// we'll get a Ctx struct in ctx. If not, we'll get an Error.
// If we use "ctx: Ctx" instead, this function will not be called
// if extraction fails, the response will come from the extractor
pub async fn mw_require_auth<B>(
    ctx: Result<Ctx>,
    req: Request<B>, 
    next: Next<B>
) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth - {ctx:?}", "MIDDLEWARE");

    ctx?; // This will handle the Ok or Err variant

    Ok(next.run(req).await)
}

// region:      --- Ctx Extractor
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        println!("->> {:<12} - Ctx", "EXTRACTOR");
        
        // Use the cookie extractor
        let cookies = parts.extract::<Cookies>().await.unwrap();
        
        // Convert to an Option<String> if we have Option<Cookie>
        let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

        // Parse token.
        let (user_id, exp, sign) = auth_token
            .ok_or_else(|| Error::AuthFailNoAuthTokenCookie)
            .and_then(parse_token)?;

        // TODO: Token components validation.

        Ok(Ctx::new(user_id))
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