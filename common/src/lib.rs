use std::clone;

use chrono:: NaiveDateTime;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub enum WebSocketMessageType {
    NewMessage,
    UserList,
    UsernameChange,
}
#[derive(Serialize, Deserialize)]
pub struct WebSocketMessage {
    pub message_type: WebSocketMessageType,
    pub message: Option<ChatMessage>,
    pub users: Option<Vec<String>>,
    pub username: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct ChatMessage{
    pub message: String,
    pub author: String,
    pub created_at: NaiveDateTime
}

