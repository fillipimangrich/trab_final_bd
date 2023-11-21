use crate::{AppState, models};
use models::game::{GameModel, CreateGameSchema};
use actix_web::{get, post, web, HttpResponse, Responder};


#[get("/games")]
pub async fn get_games(data: web::Data<AppState>) -> impl Responder{

    match sqlx::query_as::<_, GameModel>("SELECT * FROM games")
    .fetch_all(&data.db)
    .await
    {
        Ok(games) => HttpResponse::Ok().json(games),
        Err(e) => HttpResponse::NotFound().json("No games found"),
    }

}

#[post("/games/game")]
pub async fn create_game(body: web::Json<CreateGameSchema>, data: web::Data<AppState>) -> impl Responder {
    
     match sqlx::query_as::<_,GameModel>(
        "INSERT into games (name, price, genre_id, developer_id) values ($1, $2, $3, $4) returning *"       
    )   
        .bind(body.name.to_string())
        .bind(body.price)
        .bind(body.genre_id)
        .bind(body.developer_id)
        .fetch_one(&data.db)
        .await
    {
        Ok(game) => HttpResponse::Ok().json(game),

        Err(e) =>  HttpResponse::InternalServerError().json("Something wrong was happening"),
    }
}