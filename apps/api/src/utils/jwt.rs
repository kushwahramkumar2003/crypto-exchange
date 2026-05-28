use std::time::{Duration, SystemTime, UNIX_EPOCH};

use jsonwebtoken::{EncodingKey, Header, encode};
use poem::{Error, http::StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct Claims{
    pub sub:String,
    pub exp:usize
}

pub fn generate_jwt(user_id:String)->Result<String,Error>{
    let exp = (SystemTime::now() + Duration::from_secs(60 * 60))
        .duration_since(UNIX_EPOCH)
        .map_err(|_| Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))?
        .as_secs() as usize;

    let my_claims = Claims {
        sub: user_id,
        exp,
    };
    let header = Header::default();
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| String::from("secret"));
    let token = encode(&header, &my_claims, &EncodingKey::from_secret(secret.as_bytes()))
        .map_err(|_| Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(token)
}