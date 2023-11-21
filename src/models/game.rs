use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct GameModel {
    pub game_id : i32,
    pub name: String,
    pub price: f64,
    pub genre_id: i32,
    pub developer_id: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateGameSchema {
    pub name: String,
    pub price: f64,
    pub genre_id: i32,
    pub developer_id: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateGameSchema {
    pub name: String,
    pub price: f64,
    pub genre_id: i32,
    pub developer_id: i32,
}