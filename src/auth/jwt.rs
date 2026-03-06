use crate::auth::claims::Claims;
use jsonwebtoken::errors::Error;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use std::env;

pub fn get_jwt_secret() -> String {
    env::var("JWT_SECRET").expect("JWT_SECRET must be set")
}

pub fn create_jwt(claims: Claims) -> Result<String, Error> {
    let secret = get_jwt_secret();

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn verify_jwt(token: &str) -> Result<Claims, Error> {
    let secret = get_jwt_secret();

    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;

    Ok(data.claims)
}
