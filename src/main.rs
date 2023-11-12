mod routes;

use actix_web::{http::header, HttpServer, middleware::Logger, App};
use sqlx::{Pool, postgres::PgPoolOptions, Postgres};
use actix_cors::Cors;
use dotenv::dotenv;
use routes::health_route::health_checker_handler;
pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

   if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
   }

   dotenv().ok();
   env_logger::init();

   let data_base_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

   let pool:Pool<Postgres> = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&data_base_url)
        .await
    {
        Ok(pool) => {
            println!("Connected to database");
            pool
        }
        Err(err) => {
            println!("failed to connect to database");
            std::process::exit(1);
        }
    };


    HttpServer::new(move ||{
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET","POST","PUT","DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
                .app_data(actix_web::web::Data::new(AppState {db: pool.clone()}))
                .service(health_checker_handler)
                .wrap(cors)
                .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
} 
