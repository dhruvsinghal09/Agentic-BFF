use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub api_token: Option<String>,
    pub jwt: Option<String>,
}

#[derive(Deserialize)]
pub struct ChatRequest {
    pub query: String,
}

#[derive(Deserialize, Debug)]
pub struct PythonResponse {
    pub result: PythonResult,
}

#[derive(Deserialize, Debug)]
pub struct PythonResult {
    pub messages: Vec<Message>,
}

#[derive(Deserialize, Debug)]
pub struct Message {
    pub content: String,
    #[serde(rename = "type")]
    pub msg_type: String,
}
