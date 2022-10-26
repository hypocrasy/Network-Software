use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct RegisterMessage {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ChatMessage {
    pub source_name: String,
    pub target_name: String,
    pub message: String,
}