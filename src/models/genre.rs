use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct GenreModel {
    pub genre_id : i32,
    pub genre: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateGenreSchema {
    pub genre: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateGenreSchema {
    pub genre: String,
}