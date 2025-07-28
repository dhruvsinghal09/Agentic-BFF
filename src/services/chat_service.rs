use crate::models::PythonResponse;

pub fn extract_ai_answer(resp: &PythonResponse) -> Option<String> {
    if resp.result.msg_type == "ai" {
        Some(resp.result.content.clone())
    } else {
        None
    }
}
