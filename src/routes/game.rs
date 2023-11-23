use crate::{AppState, models};
use models::game::{GameModel, UpdateGameSchema, CreateGameSchema};
use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use chrono::NaiveDate;
use serde_json::json;

#[get("/games")]
pub async fn get_games(data: web::Data<AppState>) -> impl Responder{

    match sqlx::query_as::<_, GameModel>("SELECT * FROM game")
    .fetch_all(&data.db)
    .await
    {
        Ok(games) => HttpResponse::Ok().json(json!({"games":games})),
        Err(_) => HttpResponse::NotFound().json("No games found"),
    }

}

#[get("/games/{id}")]
pub async fn get_game_by_id(path: web::Path<i32>, data: web::Data<AppState>) -> impl Responder{
    let id = path.into_inner();

    match sqlx::query_as::<_, GameModel>("SELECT * FROM game WHERE game_id = $1")
    .bind(id)
    .fetch_one(&data.db)
    .await
    {
        Ok(game) => HttpResponse::Ok().json(json!({"game":game})),
        Err(_) => HttpResponse::NotFound().json("Game not found"),
    }

}

#[post("/games/game")]
pub async fn create_game(body: web::Json<CreateGameSchema>, data: web::Data<AppState>) -> impl Responder {
    
    match sqlx::query_as::<_,GameModel>(
        "INSERT INTO game VALUES(DEFAULT, $1, $2, $3, $4, $5) returning *"       
    ) 
        .bind(body.price as f64)  
        .bind(body.name.to_string())
        .bind(body.genre_id)
        .bind(body.developer_id)
        .bind(NaiveDate::parse_from_str(&body.release_date.to_string(), "%Y-%m-%d").unwrap())
        .fetch_one(&data.db)
        .await
    {
        Ok(game) => HttpResponse::Ok().json(game),

        Err(e) =>  HttpResponse::InternalServerError().json(e.to_string()),
    }
}


#[put("/games/game/{id}")]
pub async fn update_game(path: web::Path<i32>,body: web::Json<UpdateGameSchema>, data: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    
    match sqlx::query_as::<_,GameModel>(
        "UPDATE game SET name = $1, price = $2, genre_id = $3, developer_id = $4, release_date = $5 WHERE game_id = $6 RETURNING *",       
    )  
        .bind(body.name.to_string())
        .bind(body.price as f64) 
        .bind(body.genre_id)
        .bind(body.developer_id)
        .bind(NaiveDate::parse_from_str(&body.release_date.to_string(), "%Y-%m-%d").unwrap())
        .bind(id)
        .fetch_one(&data.db)
        .await
    {
        Ok(game) => HttpResponse::Ok().json(game),

        Err(e) =>  HttpResponse::InternalServerError().json(e.to_string()),
    }
}


#[delete("/games/game/{id}")]
pub async fn delete_game(path: web::Path<i32>, data: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();

    match sqlx::query("DELETE FROM game WHERE game_id = $1")
        .bind(id)
        .execute(&data.db)
        .await
    {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::NotFound().json("Not Found"),
    }
}