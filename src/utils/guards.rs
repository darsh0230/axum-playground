use axum::{
    body::Body,
    http::{header, Request, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::{types::error_types::APIError, utils::jwt::decode_jwt};

pub async fn jwt_guard(mut req: Request<Body>, next: Next) -> Result<Response, APIError> {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "))
        .ok_or(APIError {
            status_code: StatusCode::UNAUTHORIZED,
            error_message: "You are not logged in, please provide AUTH token".to_string(),
            user_message: "You are not logged in, please provide AUTH token".to_string(),
            ..Default::default()
        })?;

    let claim = decode_jwt(token.to_string())
        .map_err(|err| APIError {
            status_code: StatusCode::UNAUTHORIZED,
            error_message: err.to_string(),
            user_message: "Unauthorized.".to_string(),
            ..Default::default()
        })?
        .claims;

    req.extensions_mut().insert(claim.user_id);

    Ok(next.run(req).await)
}
