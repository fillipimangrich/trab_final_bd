use crate::{AppState, models};
use models::role::{RoleModel, CreateRoleSchema};
use actix_web::{get, post, web, HttpResponse, Responder};
use serde_json::json;

#[get("/roles")]
pub async fn get_roles(data: web::Data<AppState>) -> impl Responder{

    match sqlx::query_as::<_, RoleModel>("SELECT * FROM role")
    .fetch_all(&data.db)
    .await
    {
        Ok(roles) => HttpResponse::Ok().json(json!({"roles":roles})),
        Err(e) => HttpResponse::NotFound().json("No roles found"),
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

        Err(e) =>  HttpResponse::InternalServerError().json("Something wrong was happening"),
    }
}

#[delete("/roles/role/{id}")]
pub async fn delete_game(path: web::Path<i32>, data: web::Data<AppState>) -> impl Responder {
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