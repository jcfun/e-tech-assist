/*
 * @Author: jcfun jcfunstar@gmail.com
 * @Date: 2023-03-23 15:31:41
 * @LastEditors: jcfun jcfunstar@gmail.com
 * @LastEditTime: 2023-03-24 22:30:24
 * @FilePath: /e-tech-assist/web-server/src/routers/user.rs
 * @Description:
 */

use crate::{common::state::AppState, handlers::user::*};
use axum::{
    routing::{get, post},
    Router,
};
pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/:id",
            get(get_user_by_id).delete(delete_user).put(update_user),
        )
        .route("/", post(create_user))
}
