use axum::http::StatusCode;
use chrono::{Duration, Utc};
use dotenvy_macro::dotenv;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use super::app_error::AppError;

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    // sub: String,
    // company: String,
    exp: usize,
    iat: usize,
}

pub fn create_jwt() -> Result<String, AppError> {
    let mut date = Utc::now();
    let iat = date.timestamp() as usize;
    let expires_in = Duration::seconds(60);
    date += expires_in;
    let exp = date.timestamp() as usize;

    let claims = Claims { exp, iat };
    let secret_key: &'static str = dotenv!("SECRET");

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_key.as_bytes()),
    )
    .map_err(|_error| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"));

    return token;
}

pub fn is_token_valid(token: &str) -> Result<bool, AppError> {
    let secret: &'static str = dotenv!("SECRET");
    let key = DecodingKey::from_secret(secret.as_bytes());
    decode::<Claims>(token, &key, &Validation::new(Algorithm::HS256)).map_err(
        |error| match error.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => AppError::new(
                StatusCode::UNAUTHORIZED,
                "Your session expired, please login again",
            ),
            _ => AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong, please try again",
            ),
        },
    )?;

    Ok(true)
}
