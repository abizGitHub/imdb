use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct HelloResponse {
    pub message: String,
}

#[derive(Deserialize)]
pub struct EchoRequest {
    pub from: String,
    pub message: MessageContent,
}

#[derive(Deserialize)]
pub struct MessageContent {
    #[serde(default)]
    pub id: Option<u32>,
    pub lines: Vec<String>,
}

#[derive(Serialize)]
pub struct EchoResponse {
    pub from: String,
    pub echoed_lines: Vec<String>,
    pub message_id: Option<u32>,
}
