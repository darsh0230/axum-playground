use axum::{http::StatusCode, Extension, Json};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, Set,
};
use uuid::Uuid;

use crate::types::auth_types;
use crate::types::error_types::APIError;
use crate::utils::jwt::encode_jwt;

use bcrypt::{hash, verify, DEFAULT_COST};

pub async fn user_register(
    Extension(db): Extension<DatabaseConnection>,
    Json(user_data): Json<auth_types::UserRegisterReq>,
) -> Result<auth_types::UserRegisterRes, APIError> {
    let password_hash = hash(user_data.password, DEFAULT_COST)
        .map_err(|err| APIError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_message: err.to_string(),
            ..Default::default()
        })?
        .to_string();

    let user = entity::users::Entity::find()
        .filter(entity::users::Column::Email.eq(user_data.email.clone()))
        .one(&db)
        .await
        .map_err(|err| APIError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_message: err.to_string(),
            ..Default::default()
        })?;

    if user != None {
        return Err(APIError {
            status_code: StatusCode::BAD_REQUEST,
            error_message: "User already exists.".to_string(),
            user_message: "User already exists. Please try logging in.".to_string(),
            ..Default::default()
        });
    }

    let user_model = entity::users::ActiveModel {
        name: Set(Some(user_data.name.to_owned())),
        email: Set(Some(user_data.email.to_owned())),
        password: Set(Some(password_hash)),
        user_id: Set(Uuid::new_v4()),
        created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };
    user_model.insert(&db).await.map_err(|err| APIError {
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
        error_message: err.to_string(),
        ..Default::default()
    })?;

    Ok(auth_types::UserRegisterRes {
        name: user_data.name.to_string(),
        email: user_data.email.to_string(),
    })
}

pub async fn user_login(
    Extension(db): Extension<DatabaseConnection>,
    Json(user_data): Json<auth_types::UserLoginReq>,
) -> Result<auth_types::UserLoginRes, APIError> {
    let user = entity::users::Entity::find()
        .filter(Condition::all().add(entity::users::Column::Email.eq(user_data.email)))
        .one(&db)
        .await
        .map_err(|err| APIError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_message: err.to_string(),
            ..Default::default()
        })?
        .ok_or(APIError {
            status_code: StatusCode::BAD_REQUEST,
            error_message: "Invalid Email or password.".to_string(),
            user_message: "Invalid Email or password.".to_string(),
            ..Default::default()
        })?;

    // password verification
    let hashed_pwd = user.password.ok_or(APIError {
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
        error_message: "No hashed password found".to_string(),
        ..Default::default()
    })?;

    verify(user_data.password, &hashed_pwd).map_err(|_| APIError {
        status_code: StatusCode::BAD_REQUEST,
        error_message: "Invalid Email or password.".to_string(),
        user_message: "Invalid Email or password.".to_string(),
        ..Default::default()
    })?;

    // generate token
    let token = encode_jwt(user.user_id.to_string()).map_err(|_| APIError {
        status_code: StatusCode::BAD_REQUEST,
        error_message: "Invalid Email or password.".to_string(),
        user_message: "Invalid Email or password.".to_string(),
        ..Default::default()
    })?;

    Ok(auth_types::UserLoginRes { token })
}
