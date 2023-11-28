use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct DeveloperModel {
    pub developer_id : i32,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateDeveloperSchema {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateDeveloperSchema {
    pub name: String,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct DeveloperEarningModel {
    pub developer_id : i32,
    pub name: String,
    pub total_ganho: Option<f64>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct DeveloperSoldsModel {
    pub developer_id : i32,
    pub name: String,
    pub quantidade_de_jogos_vendidos: Option<i64>,
}
