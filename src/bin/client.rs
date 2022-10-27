use std::io::Read;

use tokio::{net::{TcpStream, tcp::{OwnedReadHalf, OwnedWriteHalf}}, io::{AsyncWriteExt, AsyncReadExt}};

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
    let mut name = String::new();
    println!("type your name");
    std::io::stdin().read_line(&mut name).ok().expect("Failed to read line");
    let len = name.len();
    let name = (&name[0..len-2]).to_string();
    let reg = RegisterMessage {
        // name: args[1].clone()
        name: name.clone()
    };

    let buf = serde_json::to_string(&reg).unwrap().into_bytes();
    so.write_u32(buf.len() as u32).await.unwrap();
    println!("$(target_name)::$(message)");
    so.write(&buf).await.unwrap();
    let (rx, tx) = so.into_split();
    tokio::spawn(recv(rx, name.clone()));
    send(tx, name.clone()).await;
    
}

async fn send(mut so: OwnedWriteHalf, name: String) {
    loop {
        let mut message = String::new();
        // print!("> ");
        std::io::stdin().read_line(&mut message).ok().expect("Failed to read line");
        let message = (&message[0..message.len()-2]).to_string();
        
        let message: Vec<_> = message.split("::").collect();
        let chat = ChatMessage {
            source_name: name.clone(),
            target_name: message[0].to_string(),
            message: message[1].to_string(),
        };
        let mes = serde_json::to_string(&chat).unwrap().into_bytes();
        
        so.write_u32(mes.len() as u32).await.unwrap();
        so.write(& mes).await.unwrap();
    }
}
async fn recv(mut so: OwnedReadHalf, name: String) {
    loop {
        let len = so.read_u32().await.unwrap();
        let mut buf = vec![0u8; len as usize];
        so.read_exact(&mut buf).await.unwrap();
        let chat: ChatMessage = serde_json::from_slice(&buf).unwrap();
        // println!("{:?}", chat);
        if chat.target_name == name && chat.source_name != name {
            println!("recv message from {}: {}", chat.source_name, chat.message);
        }
    }
}