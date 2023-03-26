use std::time::{SystemTime, UNIX_EPOCH};

use axum::{
    async_trait,
    extract::FromRequest,
    http::{Request, StatusCode},
    response::IntoResponse,
    TypedHeader,
};

use headers::{authorization::Bearer, Authorization};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use super::{errors::MyError, res::Res};

/*
 * @Author: jcfun jcfunstar@gmail.com
 * @Date: 2023-03-25 19:44:00
 * @LastEditors: jcfun jcfunstar@gmail.com
 * @LastEditTime: 2023-03-26 15:53:46
 * @FilePath: /e-tech-assist/web-server/src/common/jwt.rs
 * @Description:
 */
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    id: String,
    account: String,
    exp: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct  Token {
    token: String,
    token_type: String,
}

pub enum AuthError {
    InvalidToken,
}

pub fn encode_jwt(id: String, account: String) -> Token {
    let claims = Claims {
        id,
        account,
        exp: get_epoch() + 24 * 60 * 60,
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secret".as_ref()),
    )
    .unwrap();
    Token {
        token,
        token_type: "Bearer".to_string(),
    }
}

pub fn decode_jwt(token: String) -> Result<Claims, MyError> {
    let token_info = decode::<Claims>(
        &token,
        &DecodingKey::from_secret("secret".as_ref()),
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
        let token = decode_jwt(bearer.token().into());
        token
            .map(|claims| Ok(claims))
            .unwrap_or_else(|_| Err(AuthError::InvalidToken))
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::OK,
            Res::<()>::from_msg(StatusCode::UNAUTHORIZED, "Invalid Token"),
        )
            .into_response()
    }
}

fn get_epoch() -> usize {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    since_the_epoch.as_secs() as usize
}
