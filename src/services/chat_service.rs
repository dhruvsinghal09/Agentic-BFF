use crate::models::PythonResponse;

pub fn extract_ai_answer(resp: &PythonResponse) -> Option<String> {
    resp.result
        .messages
        .iter()
        .rev() // iterate from last to first
        .find(|m| m.msg_type == "ai")
        .map(|m| m.content.clone())
}
