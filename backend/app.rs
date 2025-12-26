use crate::{db, util::PasswordChecker};
use axum::{
    extract::Json,
    http::{StatusCode, header::SET_COOKIE},
    response::Response,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct LoginResponse {
    user_id: i32,
    name: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

/*
pub async fn login(
    Json(payload): Json<LoginRequest>,
) -> Result<Response<LoginResponse>, (StatusCode, String)> {
    let user = db::find_user(_, &payload.email).await;

    if let Some(user) = user {
        let pwdchk = PasswordChecker::from_hash(&user.password_hash);
        if pwdchk.check(&payload.password) {
            let session_id = create_session_for_user(user.id);
            let cookie = format!("session={}; HttpOnly; Path=/; Secure", session_id);
            return Ok(Response::builder()
                .header(SET_COOKIE, cookie)
                .body(Json(LoginResponse {
                    user_id: user.id,
                    name: user.name,
                }))
                .unwrap());
        }
    }
    Err((StatusCode::UNAUTHORIZED, "a".to_string()))
}

pub async fn logout(session: Session) -> impl IntoResponse {
    session.destroy();
    (StatusCode::OK, "Logged out")
}
    */
