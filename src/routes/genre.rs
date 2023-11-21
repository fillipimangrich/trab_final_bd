use crate::{AppState, models};
use models::genre::{GenreModel, CreateGenreSchema};
use actix_web::{get, post, web, HttpResponse, Responder};
use serde_json::json;

#[get("/genres")]
pub async fn get_genres(data: web::Data<AppState>) -> impl Responder{

    match sqlx::query_as::<_, GenreModel>("SELECT * FROM genre")
    .fetch_all(&data.db)
    .await
    {
        Ok(genres) => HttpResponse::Ok().json(json!({"genres":genres})),
        Err(e) => HttpResponse::NotFound().json("No genre found"),
    }

}

#[post("/genres/genre")]
pub async fn create_genre(body: web::Json<CreateGenreSchema>, data: web::Data<AppState>) -> impl Responder {
    
     match sqlx::query_as::<_,GenreModel>(
        "INSERT INTO genre VALUES(DEFAULT, $1) returning *"       
    )   
        .bind(body.genre.to_string())
        .fetch_one(&data.db)
        .await
    {
        Ok(genre) => HttpResponse::Ok().json(genre),

        Err(e) =>  HttpResponse::InternalServerError().json("Something wrong was happening"),
    }
}