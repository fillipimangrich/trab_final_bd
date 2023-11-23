use crate::{AppState, models};
use models::role::{RoleModel, UpdateRoleSchema, CreateRoleSchema};
use actix_web::{get, post, delete, put, web, HttpResponse, Responder};
use serde_json::json;

#[get("/roles")]
pub async fn get_roles(data: web::Data<AppState>) -> impl Responder{

    match sqlx::query_as::<_, RoleModel>("SELECT * FROM role")
    .fetch_all(&data.db)
    .await
    {
        Ok(roles) => HttpResponse::Ok().json(json!({"roles":roles})),
        Err(_) => HttpResponse::NotFound().json("No roles found"),
    }

}

#[get("/roles/{id}")]
pub async fn get_role_by_id(path: web::Path<i32>, data: web::Data<AppState>) -> impl Responder{
    let id = path.into_inner();

    match sqlx::query_as::<_, RoleModel>("SELECT * FROM role WHERE role_id = $1")
    .bind(id)
    .fetch_one(&data.db)
    .await
    {
        Ok(genre) => HttpResponse::Ok().json(json!({"role":genre})),
        Err(_) => HttpResponse::NotFound().json("Role not found"),
    }

}

#[post("/roles/role")]
pub async fn create_role(body: web::Json<CreateRoleSchema>, data: web::Data<AppState>) -> impl Responder {
    
     match sqlx::query_as::<_,RoleModel>(
        "INSERT INTO role VALUES(DEFAULT, $1) returning *"       
    )   
        .bind(body.name.to_string())
        .fetch_one(&data.db)
        .await
    {
        Ok(role) => HttpResponse::Ok().json(role),

        Err(e) =>  HttpResponse::InternalServerError().json(e.to_string()),
    }
}

#[put("/roles/role/{id}")]
pub async fn update_role(path: web::Path<i32>,body: web::Json<UpdateRoleSchema>, data: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    
    match sqlx::query_as::<_,RoleModel>(
        "UPDATE role SET name = $1 WHERE role_id = $2 RETURNING *",       
    )  
        .bind(body.name.to_string())
        .bind(id)
        .fetch_one(&data.db)
        .await
    {
        Ok(role) => HttpResponse::Ok().json(role),

        Err(e) =>  HttpResponse::InternalServerError().json(e.to_string()),
    }
}

#[delete("/roles/role/{id}")]
pub async fn delete_role(path: web::Path<i32>, data: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();

    match sqlx::query("DELETE FROM role WHERE role_id = $1")
        .bind(id)
        .execute(&data.db)
        .await
    {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::NotFound().json("Not Found"),
    }
}