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