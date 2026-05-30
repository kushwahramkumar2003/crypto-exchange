use jsonwebtoken::{DecodingKey, Validation, decode};
use poem::{Error, FromRequest, Result, http::StatusCode};

use crate::utils::jwt::Claims;

pub struct UserId {
    pub user_id: String,
}

impl<'a> FromRequest<'a> for UserId {
    async fn from_request(req: &'a poem::Request, _body: &mut poem::RequestBody) -> Result<Self> {
        let token_from_cookie = req
            .headers()
            .get("cookie")
            .and_then(|value| value.to_str().ok())
            .and_then(|cookie_header| {
                cookie_header
                    .split(';')
                    .map(str::trim)
                    .find_map(|part| part.strip_prefix("token="))
            });

        let token_from_auth = req
            .headers()
            .get("authorization")
            .and_then(|value| value.to_str().ok())
            .map(|auth_header| auth_header.strip_prefix("Bearer ").unwrap_or(auth_header));

        let token = token_from_cookie
            .or(token_from_auth)
            .ok_or_else(|| Error::from_string("missing token", StatusCode::UNAUTHORIZED))?;

        let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| String::from("secret"));
        let claims = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| Error::from_string("invalid token", StatusCode::UNAUTHORIZED))?;

        Ok(UserId {
            user_id: claims.claims.sub,
        })
    }
}
