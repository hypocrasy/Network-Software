use std::env;

use tokio::{net::TcpStream, io::AsyncWriteExt};

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
        name: String::from("dialer")
    };

    let buf = serde_json::to_string(&reg).unwrap().into_bytes();
    so.write_u32(buf.len() as u32).await.unwrap();
    
    so.write(&buf).await.unwrap();


    send(so).await;
    
}

async fn send(mut so: TcpStream) {
    let chat = ChatMessage {
        source_name: "dialer".to_string(),
        target_name: "listener".to_string(),
        message: "hello".to_string(),
    };
    let mes = serde_json::to_string(&chat).unwrap().into_bytes();

    // let deserde: ChatMessage = serde_json::from_slice(&mes).unwrap();
    // println!("{:?}", deserde);
    
    so.write_u32(mes.len() as u32).await.unwrap();
    so.write(& mes).await.unwrap();
    loop{

    }
}