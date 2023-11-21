use actix_web::web;
use super::game::{create_game, get_games};
use super::genre::{create_genre, get_genres};


pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(get_games)
        .service(create_game)
        .service(create_genre) // Add the `create_genre` service
        .service(get_genres); // Add the `get_genres` service
        // .service(delete_game);

    conf.service(scope);
}