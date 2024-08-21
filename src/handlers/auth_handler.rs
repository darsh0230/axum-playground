use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, Set,
};
use serde_json::json;
use uuid::Uuid;

use crate::types::auth_types;
use crate::types::error_types::APIError;

pub async fn user_register(
    Extension(db): Extension<DatabaseConnection>,
    Json(user_data): Json<auth_types::UserRegisterReq>,
) -> Result<auth_types::UserRegisterRes, APIError> {
    let user_model = entity::users::ActiveModel {
        name: Set(Some(user_data.name.to_owned())),
        email: Set(Some(user_data.email.to_owned())),
        password: Set(Some(user_data.password.to_owned())),
        uuid: Set(Uuid::new_v4()),
        created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };
    user_model.insert(&db).await.map_err(|err| APIError {
        error_message: err.to_string(),
    })?;

    Ok(auth_types::UserRegisterRes {
        name: user_data.name.to_string(),
        email: user_data.email.to_string(),
    })
}
