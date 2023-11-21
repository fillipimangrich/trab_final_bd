use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct RoleModel {
    pub role_id : i32,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateRoleSchema {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateRoleSchema {
    pub name: String,
}