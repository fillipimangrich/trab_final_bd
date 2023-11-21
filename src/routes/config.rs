use actix_web::web;
use super::game::{create_game, get_games};


pub fn config(conf: &mut web::ServiceConfig){
    let scope = web::scope("/games")
        .service(get_games)
        .service(create_game)
        .service(delete_game);

    conf.service(scope);
}