use crate::{AppState, models};
use models::role::{RoleModel, CreateRoleSchema};
use actix_web::{get, post, web, HttpResponse, Responder};


#[get("/roles")]
pub async fn get_roles(data: web::Data<AppState>) -> impl Responder{

    match sqlx::query_as::<_, RoleModel>("SELECT * FROM role")
    .fetch_all(&data.db)
    .await
    {
        Ok(roles) => HttpResponse::Ok().json(roles),
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