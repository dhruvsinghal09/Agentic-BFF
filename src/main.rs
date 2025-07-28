mod auth;
mod controllers;
mod middleware;
mod models;
mod python_client;
mod state;
mod services;

use crate::controllers::chat_controller;
use crate::controllers::init_controller;
use crate::controllers::user_controller;
use crate::middleware::{ApiTokenMiddlewareFactory, JwtMiddlewareFactory};
use crate::state::AppState;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    std::env::set_var("RUST_LOG", "info,debug,actix_web=info"); // adjust log level
    env_logger::init();
    let state = AppState::new();
    let state_data = web::Data::new(state);

    HttpServer::new(move || {
        App::new().app_data(state_data.clone()).service(
            web::scope("/api")
                .configure(init_controller::config) // /api/init (public)
                .service(
                    web::scope("")
                        .wrap(ApiTokenMiddlewareFactory)
                        .configure(user_controller::config) // /api/create_user, /api/login
                        .service(
                            web::scope("")
                                .wrap(JwtMiddlewareFactory)
                                .configure(chat_controller::config), // /api/chat
                        ),
                ),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
