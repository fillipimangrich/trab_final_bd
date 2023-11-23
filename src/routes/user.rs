use crate::{AppState, models};
use models::user::{UserModel, UpdateUserSchema, CreateUserSchema};
use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
// use chrono::NaiveDate;
use serde_json::json;

#[get("/users")]
pub async fn get_users(data: web::Data<AppState>) -> impl Responder{

    match sqlx::query_as::<_, UserModel>("SELECT * FROM users")
    .fetch_all(&data.db)
    .await
    {
        Ok(users) => HttpResponse::Ok().json(json!({"users":users})),
        Err(_) => HttpResponse::NotFound().json("No users found"),
    }

}

#[get("/users/{id}")]
pub async fn get_user_by_id(path: web::Path<i32>, data: web::Data<AppState>) -> impl Responder{
    let id = path.into_inner();

    match sqlx::query_as::<_, UserModel>("SELECT * FROM users WHERE user_id = $1")
    .bind(id)
    .fetch_one(&data.db)
    .await
    {
        Ok(user) => HttpResponse::Ok().json(json!({"user":user})),
        Err(_) => HttpResponse::NotFound().json("User not found"),
    }

}

#[post("/users/user")]
pub async fn create_user(body: web::Json<CreateUserSchema>, data: web::Data<AppState>) -> impl Responder {
    
    match sqlx::query_as::<_,UserModel>(
        "INSERT INTO users VALUES(DEFAULT, $1, $2, $3, $4) returning *"       
    ) 
        .bind(body.username.to_string())  
        .bind(body.nick_name.to_string())
        .bind(body.password.to_string())
        .bind(body.role_id)
        .fetch_one(&data.db)
        .await
    {
        Ok(user) => HttpResponse::Ok().json(user),

        Err(e) =>  HttpResponse::InternalServerError().json(e.to_string()),
    }
}


#[put("/users/user/{id}")]
pub async fn update_user(path: web::Path<i32>,body: web::Json<UpdateUserSchema>, data: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    
    match sqlx::query_as::<_,UserModel>(
        "UPDATE users SET username = $1, nick_name = $2, password = $3, role_id = $4 WHERE user_id = $5 RETURNING *",       
    )  
        .bind(body.username.to_string())  
        .bind(body.nick_name.to_string())
        .bind(body.password.to_string())
        .bind(body.role_id)
        .bind(id)
        .fetch_one(&data.db)
        .await
    {
        Ok(role) => HttpResponse::Ok().json(role),

        Err(e) =>  HttpResponse::InternalServerError().json(e.to_string()),
    }
}


#[delete("/users/user/{id}")]
pub async fn delete_user(path: web::Path<i32>, data: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();

    match sqlx::query("DELETE FROM users WHERE user_id = $1")
        .bind(id)
        .execute(&data.db)
        .await
    {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::NotFound().json("Not Found"),
    }
}