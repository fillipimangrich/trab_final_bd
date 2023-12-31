use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDate;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct OrderModel {
    pub order_id : i32,
    pub user_id : i32,
    pub game_id : i32,
    pub order_date : NaiveDate,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateOrderSchema {
    pub user_id : i32,
    pub game_id : i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateOrderSchema {
    pub user_id : i32,
    pub game_id : i32,
    pub order_date : NaiveDate,
}

