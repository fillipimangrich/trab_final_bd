use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct UserModel {
    pub user_id : i32,
    pub nickname: String,
    pub username: String,
    pub password: String,
    pub role_id: i32
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUserSchema {
    pub nickname: String,
    pub username: String,
    pub password: String,
    pub role_id: i32
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateUserSchema {
    pub nickname: String,
    pub username: String,
    pub password: String,
    pub role_id: i32
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct UserSpendingSchema {
    pub user_id: i32,
    pub username: String,
    pub total_gasto: Option<f64>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct UserGameTimeSchema {
    pub user_id: i32,
    pub username: String,
    pub total_horas_jogadas: Option<f64>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct UserGameTimePerGameSchema {
    pub user_id: i32,
    pub username: String,
    pub game_id: Option<i32>,
    pub game_name: Option<String>,
    pub horas_jogadas: Option<f64>,
}