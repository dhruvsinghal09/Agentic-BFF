use crate::{models::TokenResponse, state::AppState};
use actix_web::{web, HttpResponse};
use log::{error, info};
use uuid::Uuid;

async fn init(state: web::Data<AppState>) -> HttpResponse {
    info!("Init API called");
    let api_token = Uuid::new_v4().to_string();
    state.api_tokens.lock().unwrap().insert(api_token.clone());
    HttpResponse::Ok().json(TokenResponse {
        api_token: Some(api_token),
        jwt: None,
    })
}

// Register routes for this controller
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/init").route(web::post().to(init)));
}
