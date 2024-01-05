use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use lazy_regex::regex_captures;
use tower_cookies::Cookies;

use crate::web::AUTH_TOKEN;
use crate::{Error, Result};

pub async fn mw_require_auth<B>(
    cookies: Cookies,
    req: Request<B>, 
    next: Next<B>
) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");

    // Convert to an Option<String> if we have Option<Cookie>
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    // Parse token.
    let (user_id, exp, sign) = auth_token
    .ok_or_else(|| Error::AuthFailNoAuthTokenCookie)
    .and_then(parse_token)?;

    // TODO: Token components validation.

    Ok(next.run(req).await)
}

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