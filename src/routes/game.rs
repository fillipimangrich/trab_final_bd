use crate::{AppState, models};
use models::game::{GameModel, CreateGameSchema};
use actix_web::{get, post, delete, web, HttpResponse, Responder};


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
        "INSERT INTO games VALUES(DEFAULT, $1, $2, $3, $4) returning *"       
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

#[delete("/games/game/{id}")]
pub async fn delete_game(path: web::Path<i32>, data: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    
     match sqlx::query_as::<_,GameModel>(
        "DELETE FROM games where id = $1"       
    )   
        .bind(id)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();
    {
        Ok(game) => HttpResponse::Ok().json(),

        Err(e) =>  HttpResponse::NotFound().json("Not Found"),
    }
}