use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::IntoResponse,
};

use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{
    common::{errors::MyError, res::Res},
    config::init::get_cfg,
    models::vo::login::UserInfoVO,
};

use super::time::get_epoch;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub id: Option<String>,
    pub account: Option<String>,
    pub nickname: Option<String>,
    pub perm_codes: Option<Vec<String>>,
    pub iat: usize,
    pub exp: usize,
}

impl Claims {
    pub fn new(
        id: Option<String>,
        account: Option<String>,
        nickname: Option<String>,
        perm_codes: Option<Vec<String>>,
    ) -> Claims {
        let epoch = get_epoch();
        Claims {
            id,
            account,
            nickname,
            perm_codes,
            iat: epoch,
            exp: epoch + get_cfg().jwt.exp,
        }
    }
}

impl Default for Claims {
    fn default() -> Self {
        Claims {
            id: Some("system".into()),
            account: Some("system".into()),
            nickname: Some("system".into()),
            perm_codes: None,
            iat: 0,
            exp: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Token {
    token: String,
    #[serde(rename = "tokenType")]
    token_type: String,
}

pub enum AuthError {
    InvalidToken,
    MissingCredentials,
}

pub async fn encode_jwt(login_info: &UserInfoVO) -> Token {
    let claims = Claims::new(
        login_info.id.clone(),
        login_info.account.clone(),
        login_info.nickname.clone(),
        login_info.perm_codes.clone(),
    );
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(&get_cfg().jwt.secret.as_bytes()),
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
        &DecodingKey::from_secret(&get_cfg().jwt.secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    );
    token_info
        .map(|token_data| {
            info!("claims ================> {:?}", token_data.claims);
            Ok(token_data.claims)
        })
        .unwrap_or_else(|err| {
            info!("token error: {:?}", err);
            Err(MyError::AxumError("token error".into()))
        })
}

// // 提取器
// #[async_trait]
// impl<S, B> FromRequest<S, B> for Claims
// where
//     B: Send + 'static,
//     S: Send + Sync,
// {
//     type Rejection = AuthError;

//     async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
//         let TypedHeader(Authorization(bearer)) =
//             TypedHeader::<Authorization<Bearer>>::from_request(req, state)
//                 .await
//                 .map_err(|_| AuthError::InvalidToken)?;
//         let token = decode_jwt(bearer.token().into()).await;
//         token
//             .map(|claims| Ok(claims))
//             .unwrap_or_else(|_| Err(AuthError::InvalidToken))
//     }
// }

/// 中间件
#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let token = parts.headers.get("Authorization");
        match token {
            Some(token) => {
                let token = token.to_str().unwrap();
                let token = token.replace("Bearer ", "");
                let token = decode_jwt(token).await;
                token
                    .map(|claims| {
                        parts.extensions.insert(claims.clone());
                        Ok(claims)
                    })
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
                Res::<()>::from_fail(StatusCode::UNAUTHORIZED, "Invalid Token"),
            )
                .into_response(),
            AuthError::MissingCredentials => (
                StatusCode::OK,
                Res::<()>::from_fail(StatusCode::UNAUTHORIZED, "Missing Credentials"),
            )
                .into_response(),
        }
    }
}
