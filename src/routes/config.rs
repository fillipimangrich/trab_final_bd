use actix_web::web;
use super::game::{create_game, get_games};
use super::genre::{create_genre, get_genres};
use super::role::{create_role, get_roles};


pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(get_games)
        .service(create_game)
        .service(create_genre) 
        .service(get_genres) 
        .service(create_role) 
        .service(get_roles);
        // .service(delete_game);

    conf.service(scope);
}