use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct UserModel {
    pub name: String,
    pub email: String,
    pub id: Uuid,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserMicroModel {
    pub name: String,
    pub id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct LoginUserModel {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginUserResponseModel {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateUserModel {
    pub name: String,
}
