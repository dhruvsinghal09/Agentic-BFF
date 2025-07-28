use crate::{
    auth::{generate_jwt, hash_password, verify_password},
    models::{CreateUserRequest, LoginRequest, TokenResponse},
    state::AppState,
};
use actix_web::{web, HttpResponse};
use log::{error, info};
use rusqlite::params;
use serde_json::json;

async fn create_user(
    state: web::Data<AppState>,
    payload: web::Json<CreateUserRequest>,
) -> HttpResponse {
    let conn = state.db_conn.lock().unwrap();
    let hashed = hash_password(&payload.password);
    info!("Creating user: {}", payload.username);
    info!("Hashed password: {}", hashed);

    let _ = conn.execute(
        "INSERT INTO users (username, password_hash) VALUES (?1, ?2)",
        params![payload.username, hashed],
    );

    HttpResponse::Ok().json(json!({"status": "created"}))
}

async fn login(state: web::Data<AppState>, payload: web::Json<LoginRequest>) -> HttpResponse {
    let conn = state.db_conn.lock().unwrap();
    let mut stmt = conn
        .prepare("SELECT password_hash FROM users WHERE username=?1")
        .unwrap();
    let result: Result<String, _> = stmt.query_row([payload.username.clone()], |row| row.get(0));

    match result {
        Ok(hash) if verify_password(&payload.password, &hash) => {
            let jwt = generate_jwt(&payload.username, &state.jwt_secret);
            HttpResponse::Ok().json(TokenResponse {
                api_token: None,
                jwt: Some(jwt),
            })
        }
        _ => HttpResponse::Unauthorized().finish(),
    }
}

// Register routes for this controller
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/create_user").route(web::post().to(create_user)))
        .service(web::resource("/login").route(web::post().to(login)));
}
