use crate::{AppState, models};
use models::developer::{DeveloperModel, CreateDeveloperSchema, UpdateDeveloperSchema};
use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use serde_json::json;

#[get("/developers")]
pub async fn get_developers(data: web::Data<AppState>) -> impl Responder{

    match sqlx::query_as::<_, DeveloperModel>("SELECT * FROM developer")
    .fetch_all(&data.db)
    .await
    {
        Ok(developers) => HttpResponse::Ok().json(json!({"developers":developers})),

        Err(_) => HttpResponse::NotFound().json("No developer found"),
    }

}

#[get("/developers/{id}")]
pub async fn get_developer_by_id(path: web::Path<i32>, data: web::Data<AppState>) -> impl Responder{
    let id = path.into_inner();

    match sqlx::query_as::<_, DeveloperModel>("SELECT * FROM developer WHERE developer_id = $1")
    .bind(id)
    .fetch_one(&data.db)
    .await
    {
        Ok(developer) => HttpResponse::Ok().json(json!({"game":developer})),

        Err(_) => HttpResponse::NotFound().json("Developer not found"),
    }

}

#[post("/developers/developer")]
pub async fn create_developer(body: web::Json<CreateDeveloperSchema>, data: web::Data<AppState>) -> impl Responder {
    
     match sqlx::query_as::<_,DeveloperModel>(
        "INSERT INTO developer VALUES(DEFAULT, $1) returning *"       
    )   
        .bind(body.name.to_string())
        .fetch_one(&data.db)
        .await
    {
        Ok(developer) => HttpResponse::Ok().json(developer),

        Err(_) =>  HttpResponse::InternalServerError().json("Something wrong was happening"),
    }
}

#[put("/developers/developer/{id}")]
pub async fn update_developer(path: web::Path<i32>,body: web::Json<UpdateDeveloperSchema>, data: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    
    match sqlx::query_as::<_,DeveloperModel>(
        "UPDATE developer SET name = $1 WHERE developer_id = $2 RETURNING *",       
    )  
        .bind(body.name.to_string())
        .bind(id)
        .fetch_one(&data.db)
        .await
    {
        Ok(game) => HttpResponse::Ok().json(game),

        Err(e) =>  HttpResponse::InternalServerError().json(e.to_string()),
    }
}

#[delete("/developers/developer/{id}")]
pub async fn delete_developer(path: web::Path<i32>, data: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();

    match sqlx::query("DELETE FROM developer WHERE developer_id = $1")
        .bind(id)
        .execute(&data.db)
        .await
    {
        Ok(_) => HttpResponse::NoContent().finish(),

        Err(_) => HttpResponse::NotFound().json("Not Found"),
    }
}