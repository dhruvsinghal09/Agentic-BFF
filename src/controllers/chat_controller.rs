use crate::models::{ChatRequest, PythonResponse};
use crate::services::chat_service::extract_ai_answer;
use actix_web::{web, HttpRequest, HttpResponse};
use log::info;
use serde_json::json;

async fn chat(req: HttpRequest, payload: web::Json<ChatRequest>) -> HttpResponse {
    let client = reqwest::Client::new();
    let python_url = "http://127.0.0.1:8000/rag/query";

    let session_id = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .map(|auth| auth.strip_prefix("Bearer ").unwrap_or(auth).to_string());

    let mut request_body = json!({
        "query": payload.query,
    });

    if let Some(token) = session_id {
        request_body["session_id"] = json!(token);
    }
    info!(
        "Sending request to Python service: {:?}",
        request_body.clone()
    );

    let response = client.post(python_url).json(&request_body).send().await;

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                match resp.json::<PythonResponse>().await {
                    Ok(python_resp) => {
                        info!("Received response from Python service: {:?}", python_resp);
                        if let Some(answer) = extract_ai_answer(&python_resp) {
                            HttpResponse::Ok().json(json!({ "answer": answer }))
                        } else {
                            HttpResponse::Ok().json(json!({ "answer": "No AI response found" }))
                        }
                    }
                    Err(e) => HttpResponse::InternalServerError()
                        .body(format!("Failed to parse Python response: {e}")),
                }
            } else {
                HttpResponse::BadGateway().body("Python service returned an error")
            }
        }
        Err(e) => HttpResponse::InternalServerError()
            .body(format!("Failed to contact Python service: {e}")),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/chat").route(web::post().to(chat)));
}
