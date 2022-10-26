use std::env;

use tokio::{net::TcpStream, io::{AsyncWriteExt, AsyncReadExt}};

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

#[tokio::main]
async fn main() {
    // let args: Vec<String> = env::args().collect();
    let mut so = TcpStream::connect("127.0.0.1:11451").await.unwrap();
    let reg = RegisterMessage {
        // name: args[1].clone()
        name: String::from("listener")
    };

    let buf = serde_json::to_string(&reg).unwrap().into_bytes();
    so.write_u32(buf.len() as u32).await.unwrap();
    // let buf = [0u8, 10];
    // so.write(&buf).await.unwrap();
    
    so.write(&buf).await.unwrap();

    recv(so).await
}

async fn recv(mut so: TcpStream) {
    loop {
        let len = so.read_u32().await.unwrap();
        let mut buf = vec![0u8; len as usize];
        so.read_exact(&mut buf).await.unwrap();
        let chat: ChatMessage = serde_json::from_slice(&buf).unwrap();
        if chat.target_name == "listener" {
            println!("{}", chat.message);
        }
    }
}