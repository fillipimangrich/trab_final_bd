use crate::{AppState, models};
use models::session::{SessionModel, UpdateSessionSchema, CreateSessionSchema};
use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use chrono::NaiveDate;
use serde_json::json;

#[get("/sessions")]
pub async fn get_sessions(data: web::Data<AppState>) -> impl Responder{

    match sqlx::query_as::<_, SessionModel>("SELECT * FROM session")
    .fetch_all(&data.db)
    .await
    {
        Ok(sessions) => HttpResponse::Ok().json(json!({"sessions":sessions})),
        Err(_) => HttpResponse::NotFound().json("No session found"),
    }

}

#[get("/sessions/{id}")]
pub async fn get_session_by_id(path: web::Path<i32>, data: web::Data<AppState>) -> impl Responder{
    let id = path.into_inner();

    match sqlx::query_as::<_, SessionModel>("SELECT * FROM session WHERE session_id = $1")
    .bind(id)
    .fetch_one(&data.db)
    .await
    {
        Ok(session) => HttpResponse::Ok().json(json!({"session":session})),
        Err(_) => HttpResponse::NotFound().json("User not found"),
    }

}

#[post("/sessions/session")]
pub async fn create_session(body: web::Json<CreateSessionSchema>, data: web::Data<AppState>) -> impl Responder {
    
    match sqlx::query_as::<_,SessionModel>(
        "INSERT INTO session VALUES(DEFAULT, $1, $2, $3, DEFAULT) returning *"       
    ) 
        .bind(body.user_id)  
        .bind(body.game_id)
        .bind(body.duration)
        .fetch_one(&data.db)
        .await
    {
        Ok(session) => HttpResponse::Ok().json(session),

        Err(e) =>  HttpResponse::InternalServerError().json(e.to_string()),
    }
}


#[put("/sessions/session/{id}")]
pub async fn update_session(path: web::Path<i32>,body: web::Json<UpdateSessionSchema>, data: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    
    match sqlx::query_as::<_,SessionModel>(
        "UPDATE session SET user_id = $1, game_id = $2, duration = $3, session_date = $4 WHERE session_id = $5 RETURNING *",       
    )  
        .bind(body.user_id)  
        .bind(body.game_id)
        .bind(body.duration)
        .bind(NaiveDate::parse_from_str(&body.session_date.to_string(), "%Y-%m-%d").unwrap())
        .bind(id)
        .fetch_one(&data.db)
        .await
    {
        Ok(role) => HttpResponse::Ok().json(role),

        Err(e) =>  HttpResponse::InternalServerError().json(e.to_string()),
    }
}


#[delete("/sessions/session/{id}")]
pub async fn delete_session(path: web::Path<i32>, data: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();

    match sqlx::query("DELETE FROM session WHERE session_id = $1")
        .bind(id)
        .execute(&data.db)
        .await
    {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::NotFound().json("Not Found"),
    }
}