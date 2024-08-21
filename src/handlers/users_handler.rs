// use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
// use chrono::Utc;
// use sea_orm::{
//     ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, Set,
// };
// use serde_json::json;
// use uuid::Uuid;

// use crate::{
//     models::user_model::{LoginUserModel, LoginUserResponseModel, UserModel},
//     types::error_types::APIError,
//     types::users_types::{CreateUserReq, CreateUserRes}, // utils::{api_error::APIError, jwt::encode_jwt},
// };

// pub async fn create_user(
//     Extension(db): Extension<DatabaseConnection>,
//     Json(user_data): Json<CreateUserReq>,
// ) -> Result<CreateUserRes, APIError> {
//     // if user_data.is_none() {
//     //     return Err((StatusCode::BAD_REQUEST, Json(json!({"test": "tesrs"}))));
//     // }

//     // let user = entity::user::Entity::find()
//     //     .filter(entity::user::Column::Email.eq(user_data.email.clone()))
//     //     .one(&db)
//     //     .await
//     //     .map_err(|err| APIError {
//     //         message: err.to_string(),
//     //         status_code: StatusCode::INTERNAL_SERVER_ERROR,
//     //         error_code: Some(50),
//     //     })?;

//     // if user != None {
//     //     return Err(APIError {
//     //         message: "User exists".to_owned(),
//     //         status_code: StatusCode::CONFLICT,
//     //         error_code: Some(40),
//     //     });
//     // }

//     // let user_model = entity::user::ActiveModel {
//     //     name: Set(user_data.name.to_owned()),
//     //     email: Set(user_data.email.to_owned()),
//     //     password: Set(user_data.password.to_owned()),
//     //     uuid: Set(Uuid::new_v4()),
//     //     created_at: Set(Utc::now().naive_utc()),
//     //     ..Default::default()
//     // };
//     // user_model.insert(&db).await.map_err(|err| APIError {
//     //     message: err.to_string(),
//     //     status_code: StatusCode::INTERNAL_SERVER_ERROR,
//     //     error_code: Some(50),
//     // })?;

//     Ok(CreateUserRes {
//         message: "success".to_string(),
//         data: "ok success".to_string(),
//     })
// }
