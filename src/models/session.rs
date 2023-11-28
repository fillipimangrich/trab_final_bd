use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDate;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct SessionModel {
    pub session_id : i32,
    pub user_id: i32,
    pub game_id: i32,
    pub duration: f64,
    pub session_date: NaiveDate
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateSessionSchema {
    pub user_id: i32,
    pub game_id: i32,
    pub duration: f64,
    pub session_date: NaiveDate

}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateSessionSchema {
    pub user_id: i32,
    pub game_id: i32,
    pub duration: f64,
    pub session_date: NaiveDate

}
