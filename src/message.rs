use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterMessage {
    pub ID: String,
    //pub ketword:String,
}

#[derive(Serialize, Deserialize, Debug)]
pub  struct ChatMessage {
    pub source_ID: String,
    pub target_ID: String,
    pub ctr:u16,
    pub message: String,
    pub message_type:u16,
}