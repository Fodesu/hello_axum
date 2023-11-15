use axum::{middleware::Next, response::Response, http::Request};
use tower_cookies::Cookies;

use crate::{web::AUTH_TOKEN, Error};

pub async fn mw_require_auth<B>(
    cookies : Cookies,
    req : Request<B>,
    next : Next<B>
) -> Result<Response, Error>{

    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    auth_token.ok_or(Error::AuthFailNoAuthTokenCookie)?;

    Ok(next.run(req).await)
}

