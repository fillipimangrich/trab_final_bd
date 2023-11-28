use crate::{AppState, models};
use models::user::{UserModel, UpdateUserSchema, CreateUserSchema, UserSpendingSchema, UserGameTimeSchema, UserGameTimePerGameSchema};
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

#[get("/user/{id}")]
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
        .bind(body.nickname.to_string())
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
        "UPDATE users SET username = $1, nickname = $2, password = $3, role_id = $4 WHERE user_id = $5 RETURNING *",       
    )  
        .bind(body.username.to_string())  
        .bind(body.nickname.to_string())
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

#[get("/users/spending")]
pub async fn get_spending_by_all_users(data: web::Data<AppState>) -> impl Responder{

    match sqlx::query_as::<_, UserSpendingSchema>(
    "SELECT
        u.user_id,
        u.username,
        COALESCE(SUM(g.price),0) AS total_gasto
    FROM
        users u
        left JOIN orders o ON u.user_id = o.user_id
        LEFT JOIN game g ON o.game_id = g.game_id
    GROUP BY
        u.user_id, u.username
    ORDER BY
        u.user_id")
    .fetch_all(&data.db)
    .await
    {
        Ok(users) => HttpResponse::Ok().json(json!({"spending":users})),
        Err(e) => HttpResponse::NotFound().json(e.to_string()),
    }
}

#[get("/users/spending/{id}")]
pub async fn get_user_spending_by_id(path: web::Path<i32>, data: web::Data<AppState>) -> impl Responder{
    let id = path.into_inner();

    match sqlx::query_as::<_,UserSpendingSchema>(
        "SELECT
            u.user_id,
            u.username,
            COALESCE(SUM(g.price),0) AS total_gasto
        FROM
            users u
            LEFT JOIN orders o ON u.user_id = o.user_id
            LEFT JOIN game g ON o.game_id = g.game_id
        WHERE u.user_id = $1
        GROUP BY
            u.user_id, u.username
        ORDER BY
            u.user_id")
    .bind(id)
    .fetch_one(&data.db)
    .await
    {
        Ok(user) => HttpResponse::Ok().json(json!({"user":user})),
        Err(e) => HttpResponse::NotFound().json(e.to_string()),
    }
}

#[get("/users/gametime")]
pub async fn get_gametime_by_all_users(data: web::Data<AppState>) -> impl Responder{

    match sqlx::query_as::<_, UserGameTimeSchema>(
    "SELECT
        u.user_id,
        u.username,
        COALESCE(SUM(s.duration), 0) AS total_horas_jogadas
    FROM
        users u
        LEFT JOIN session s ON u.user_id = s.user_id
    GROUP BY
        u.user_id, u.username;")
    .fetch_all(&data.db)
    .await
    {
        Ok(users) => HttpResponse::Ok().json(json!({"game_time":users})),
        Err(e) => HttpResponse::NotFound().json(e.to_string()),
    }
}

#[get("/users/gametime_per_game")]
pub async fn get_gametime_by_all_users_per_game(data: web::Data<AppState>) -> impl Responder{

    match sqlx::query_as::<_, UserGameTimePerGameSchema>(
    "SELECT
        u.user_id,
        u.username,
        G.game_id,
        g.name AS game_name,
        COALESCE(SUM(s.duration), 0) AS horas_jogadas
    FROM
        users u
        LEFT JOIN session s ON u.user_id = s.user_id
        LEFT JOIN game g ON s.game_id = g.game_id
    GROUP BY
        u.user_id, u.username, G.game_id, g.name;")
    .fetch_all(&data.db)
    .await
    {
        Ok(users) => HttpResponse::Ok().json(json!({"games":users})),
        Err(e) => HttpResponse::NotFound().json(e.to_string()),
    }
}