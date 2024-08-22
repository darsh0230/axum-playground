use axum::{http::StatusCode, Extension, Json};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, Set,
};
use uuid::Uuid;

use crate::types::auth_types;
use crate::types::error_types::APIError;
use crate::utils::jwt::encode_jwt;

pub async fn user_register(
    Extension(db): Extension<DatabaseConnection>,
    Json(user_data): Json<auth_types::UserRegisterReq>,
) -> Result<auth_types::UserRegisterRes, APIError> {
    let user = entity::users::Entity::find()
        .filter(entity::users::Column::Email.eq(user_data.email.clone()))
        .one(&db)
        .await
        .map_err(|err| APIError {
            error_message: err.to_string(),
        })?;

    if user != None {
        return Err(APIError {
            error_message: "User exists".to_owned(),
        });
    }

    let user_model = entity::users::ActiveModel {
        name: Set(Some(user_data.name.to_owned())),
        email: Set(Some(user_data.email.to_owned())),
        password: Set(Some(user_data.password.to_owned())),
        user_id: Set(Uuid::new_v4()),
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

pub async fn user_login(
    Extension(db): Extension<DatabaseConnection>,
    Json(user_data): Json<auth_types::UserLoginReq>,
) -> Result<auth_types::UserLoginRes, APIError> {
    let user = entity::users::Entity::find()
        .filter(
            Condition::all()
                .add(entity::users::Column::Email.eq(user_data.email))
                .add(entity::users::Column::Password.eq(user_data.password)),
        )
        .one(&db)
        .await
        .map_err(|err| APIError {
            error_message: err.to_string(),
        })?
        .ok_or(APIError {
            error_message: "User Not Found".to_owned(),
        })?;

    let token = encode_jwt(user.user_id.to_string()).map_err(|_| APIError {
        error_message: "Failed to login".to_owned(),
    })?;

    Ok(auth_types::UserLoginRes { token })
}
