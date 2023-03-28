use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
};

use axum::{
    async_trait,
    extract::{FromRequest, FromRequestParts},
    http::{request::Parts, Request, StatusCode},
    response::IntoResponse,
    TypedHeader,
};

use headers::{authorization::Bearer, Authorization};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::common::{errors::MyError, res::Res};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    id: String,
    account: String,
    iat: usize,
    exp: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Key {
    encode_key: String,
    decode_key: String,
}

impl Key {
    pub fn new() -> Key {
        let encode_key = env::var("JWT_ENCODE_KEY").expect("JWT_ENCODE_KEY is not found");
        let decode_key = env::var("JWT_DECODE_KEY").expect("JWT_DECODE_KEY is not found");
        Key {
            encode_key,
            decode_key,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Token {
    token: String,
    token_type: String,
}

pub enum AuthError {
    InvalidToken,
    MissingCredentials,
}

pub async fn encode_jwt(id: String, account: String) -> Token {
    let claims = Claims {
        id,
        account,
        iat: get_epoch(),
        exp: get_epoch() + 30 * 24 * 60 * 60,
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(Key::new().encode_key.as_ref()),
    )
    .unwrap();
    Token {
        token,
        token_type: "Bearer".to_string(),
    }
}

pub async fn decode_jwt(token: String) -> Result<Claims, MyError> {
    let token_info = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(Key::new().decode_key.as_ref()),
        &Validation::new(Algorithm::HS256),
    );
    token_info
        .map(|token_data| Ok(token_data.claims))
        .unwrap_or_else(|err| {
            println!("token error: {:?}", err);
            Err(MyError::AxumError("token error".into()))
        })
}

#[async_trait]
impl<S, B> FromRequest<S, B> for Claims
where
    B: Send + 'static,
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req, state)
                .await
                .map_err(|_| AuthError::InvalidToken)?;
        let token = decode_jwt(bearer.token().into()).await;
        token
            .map(|claims| Ok(claims))
            .unwrap_or_else(|_| Err(AuthError::InvalidToken))
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let headers = parts.headers.clone();
        let token = headers.get("Authorization");
        match token {
            Some(token) => {
                let token = token.to_str().unwrap();
                let token = token.replace("Bearer ", "");
                let token = decode_jwt(token.into()).await;
                token
                    .map(|claims| Ok(claims))
                    .unwrap_or_else(|_| Err(AuthError::InvalidToken))
            }
            None => Err(AuthError::MissingCredentials),
        }
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AuthError::InvalidToken => (
                StatusCode::OK,
                Res::<()>::from_msg(StatusCode::UNAUTHORIZED, "Invalid Token"),
            )
                .into_response(),
            AuthError::MissingCredentials => (
                StatusCode::OK,
                Res::<()>::from_msg(StatusCode::UNAUTHORIZED, "Missing Credentials"),
            )
                .into_response(),
        }
    }
}

fn get_epoch() -> usize {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    since_the_epoch.as_secs() as usize
}
