use crate::{AppState, models};
use models::developer::{DeveloperModel, CreateDeveloperSchema, 
    UpdateDeveloperSchema, DeveloperEarningModel, DeveloperSoldsModel};
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

#[get("/developer/{id}")]
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

#[get("/developers/earnings")]
pub async fn get_earnings_by_all_developers(data: web::Data<AppState>) -> impl Responder{

    match sqlx::query_as::<_, DeveloperEarningModel>(
    "SELECT
        d.developer_id,
        d.name,
        COALESCE(SUM(g.price),0) AS total_ganho
    FROM
        developer d
        LEFT JOIN game g ON d.developer_id = g.developer_id
        LEFT JOIN orders o ON g.game_id = o.game_id
    GROUP BY
        d.developer_id, d.name
    ORDER BY 
        d.developer_id;")
    .fetch_all(&data.db)
    .await
    {
        Ok(developers) => HttpResponse::Ok().json(json!({"earnings":developers})),
        Err(e) => HttpResponse::NotFound().json(e.to_string()),
    }
}

#[get("/developers/earnings/{id}")]
pub async fn get_developer_earnings_by_id(path: web::Path<i32>, data: web::Data<AppState>) -> impl Responder{
    let id = path.into_inner();

    match sqlx::query_as::<_,DeveloperEarningModel>(
        "SELECT
            d.developer_id,
            d.name,
            COALESCE(SUM(g.price),0) AS total_ganho
        FROM
            developer d
            LEFT JOIN game g ON d.developer_id = g.developer_id
            LEFT JOIN orders o ON g.game_id = o.game_id
        WHERE d.developer_id = $1
        GROUP BY
            d.developer_id, d.name
        ORDER BY 
            d.developer_id;")
    .bind(id)
    .fetch_one(&data.db)
    .await
    {
        Ok(developer) => HttpResponse::Ok().json(json!({"spending":developer})),
        Err(e) => HttpResponse::NotFound().json(e.to_string()),
    }
}

#[get("/developers/solds")]
pub async fn get_solds_by_all_developers(data: web::Data<AppState>) -> impl Responder{

    match sqlx::query_as::<_, DeveloperSoldsModel>(
    "SELECT
        d.developer_id,
        d.name,
        COALESCE(COUNT(DISTINCT o.game_id),0) AS quantidade_de_jogos_vendidos
    FROM
        developer d
        LEFT JOIN game g ON d.developer_id = g.developer_id
        LEFT JOIN orders o ON g.game_id = o.game_id
    GROUP BY
        d.developer_id, d.name;")
    .fetch_all(&data.db)
    .await
    {
        Ok(developers) => HttpResponse::Ok().json(json!({"solds":developers})),
        Err(e) => HttpResponse::NotFound().json(e.to_string()),
    }
}

#[get("/developers/solds/{id}")]
pub async fn get_developer_solds_by_id(path: web::Path<i32>, data: web::Data<AppState>) -> impl Responder{
    let id = path.into_inner();

    match sqlx::query_as::<_,DeveloperSoldsModel>(
    "SELECT
        d.developer_id,
        d.name,
        COALESCE(COUNT(DISTINCT o.game_id),0) AS quantidade_de_jogos_vendidos
    FROM
        developer d
        LEFT JOIN game g ON d.developer_id = g.developer_id
        LEFT JOIN orders o ON g.game_id = o.game_id
    WHERE d.developer_id = $1
    GROUP BY
        d.developer_id, d.name;")
    .bind(id)
    .fetch_one(&data.db)
    .await
    {
        Ok(developer) => HttpResponse::Ok().json(json!({"spending":developer})),
        Err(e) => HttpResponse::NotFound().json(e.to_string()),
    }
}