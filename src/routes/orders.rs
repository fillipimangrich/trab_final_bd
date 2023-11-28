use crate::{AppState, models};
use models::orders::{OrderModel, UpdateOrderSchema, CreateOrderSchema};
use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use chrono::NaiveDate;
use serde_json::json;

#[get("/orders")]
pub async fn get_orders(data: web::Data<AppState>) -> impl Responder{

    match sqlx::query_as::<_, OrderModel>("SELECT * FROM orders")
    .fetch_all(&data.db)
    .await
    {
        Ok(orders) => HttpResponse::Ok().json(json!({"orders":orders})),
        Err(_) => HttpResponse::NotFound().json("No orders found"),
    }

}

#[get("/orders/{id}")]
pub async fn get_order_by_id(path: web::Path<i32>, data: web::Data<AppState>) -> impl Responder{
    let id = path.into_inner();

    match sqlx::query_as::<_, OrderModel>("SELECT * FROM orders WHERE order_id = $1")
    .bind(id)
    .fetch_one(&data.db)
    .await
    {
        Ok(order) => HttpResponse::Ok().json(json!({"orders":order})),
        Err(_) => HttpResponse::NotFound().json("orders not found"),
    }

}


#[post("/orders/order")]
pub async fn create_order(body: web::Json<CreateOrderSchema>, data: web::Data<AppState>) -> impl Responder {
    
    match sqlx::query_as::<_,OrderModel>(
        "INSERT INTO orders VALUES(DEFAULT, $1, $2, DEFAULT) returning *"       
    ) 
        .bind(body.user_id)  
        .bind(body.game_id)
        .fetch_one(&data.db)
        .await
    {
        Ok(order) => HttpResponse::Ok().json(order),

        Err(e) =>  HttpResponse::InternalServerError().json(e.to_string()),
    }
}

#[put("/orders/order/{id}")]
pub async fn update_order(path: web::Path<i32>,body: web::Json<UpdateOrderSchema>, data: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    
    match sqlx::query_as::<_,OrderModel>(
        "UPDATE orders SET user_id = $1, game_id = $2, order_date = $3 WHERE order_id = $4 RETURNING *",       
    )  
        .bind(body.user_id)
        .bind(body.game_id) 
        .bind(NaiveDate::parse_from_str(&body.order_date.to_string(), "%Y-%m-%d").unwrap())
        .bind(id)
        .fetch_one(&data.db)
        .await
    {
        Ok(order) => HttpResponse::Ok().json(order),

        Err(e) =>  HttpResponse::InternalServerError().json(e.to_string()),
    }
}


#[delete("/orders/order/{id}")]
pub async fn delete_order(path: web::Path<i32>, data: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();

    match sqlx::query("DELETE FROM orders WHERE order_id = $1")
        .bind(id)
        .execute(&data.db)
        .await
    {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::NotFound().json("Not Found"),
    }
}