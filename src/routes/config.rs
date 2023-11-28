use actix_web::web;
use super::game::{create_game, get_games, get_game_by_id, update_game, delete_game};
use super::genre::{create_genre, get_genres, get_genre_by_id, update_genre, delete_genre};
use super::role::{create_role, get_roles, get_role_by_id, update_role, delete_role};
use super::developer::{create_developer, get_developers, get_developer_by_id, update_developer, 
    delete_developer,get_earnings_by_all_developers, get_developer_earnings_by_id,
    get_solds_by_all_developers, get_developer_solds_by_id};
use super::user::{create_user, get_users, get_user_by_id, update_user, delete_user,
     get_spending_by_all_users, get_user_spending_by_id, get_gametime_by_all_users,
    get_gametime_by_all_users_per_game};
use super::session::{create_session, get_sessions, get_session_by_id, update_session, delete_session,};
use super::orders::{create_order, get_orders, get_order_by_id, update_order, delete_order};


pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(get_games)
        .service(get_game_by_id)
        .service(create_game)
        .service(update_game)
        .service(delete_game)

        .service(create_genre) 
        .service(get_genres)
        .service(get_genre_by_id)
        .service(update_genre)
        .service(delete_genre) 

        .service(create_role) 
        .service(get_roles)
        .service(get_role_by_id)
        .service(update_role)
        .service(delete_role)

        .service(create_developer) 
        .service(get_developers)
        .service(get_developer_by_id)
        .service(update_developer)
        .service(delete_developer)
        .service(get_earnings_by_all_developers)
        .service(get_developer_earnings_by_id)
        .service(get_developer_solds_by_id)
        .service(get_solds_by_all_developers)

        .service(create_user) 
        .service(get_users)
        .service(get_user_by_id)
        .service(update_user)
        .service(delete_user)
        .service(get_spending_by_all_users)
        .service(get_user_spending_by_id)
        .service(get_gametime_by_all_users)
        .service(get_gametime_by_all_users_per_game)

        .service(create_session) 
        .service(get_sessions)
        .service(get_session_by_id)
        .service(update_session)
        .service(delete_session)

        .service(create_order) 
        .service(get_orders)
        .service(get_order_by_id)
        .service(update_order)
        .service(delete_order)
        ;


    conf.service(scope);
}