mod message;

use std::{sync::{Arc},};

use tokio::{net::{TcpListener, tcp::{OwnedWriteHalf, OwnedReadHalf}, TcpStream}, sync::Mutex, io::{AsyncReadExt, AsyncWriteExt}};
use message::*;

#[derive(Debug)]
struct Peer {
    name: String,
    tx: Arc<Mutex<OwnedWriteHalf>>,
}

impl PartialEq for Peer {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Eq for Peer {}

struct AllPeers {
    all: Vec<Peer>
}

lazy_static::lazy_static! {
    static ref ALL: Mutex<AllPeers> = Mutex::new(AllPeers { all: vec![] });
}

#[tokio::main]
async fn main() {
    // let reg = RegisterMessage{
        // name: String::from("cb"),
    // };
    // println!("{}", serde_json::to_string(&reg).unwrap());
    let listener = TcpListener::bind("127.0.0.1:11451").await.unwrap();
    
    loop {
        let (stream, _addr) = listener.accept().await.unwrap();
        tokio::spawn(new_commer(stream));
    }
}

async fn new_commer(stream: TcpStream) {
    let (mut rx, tx) = stream.into_split();
    let len = rx.read_u32().await.unwrap();
    let mut buf = vec![0u8; len as usize];
    rx.read_exact(&mut buf).await.unwrap();
    let reg: RegisterMessage = serde_json::from_slice(&buf).unwrap();
    dbg!(&reg);
    let peer = Peer{
        name: reg.name.clone(),
        tx: Arc::new(Mutex::new(tx))
    };
    let mut all = ALL.lock().await;
    // 如果 发现重名, 则返回失败消息
    if all.all.contains(&peer) {
        let fail_message = ChatMessage {
            source_name: reg.name.clone(),
            target_name: reg.name.clone(),
            message: reg.name.clone(),  
        };
        let buf = serde_json::to_string(&fail_message).unwrap().into_bytes();
        let len: u32 = buf.len() as u32;
        let mut tx = peer.tx.lock().await;
        tx.write_u32(len).await.unwrap();
        tx.write_all(&buf).await.unwrap();
        tx.shutdown().await.unwrap();
        return;
    }
    
    all.all.push(peer);
    drop(all);
    // serve 会在掉线时结束
    serve(rx, reg.name.clone()).await;

}


async fn serve(mut rx: OwnedReadHalf, name: String) {
    loop {
        println!("serve({})", name);
        let len = rx.read_u32().await.unwrap();
        let mut buf = vec![0u8; len as usize];
        rx.read_exact(&mut buf).await.unwrap();
        println!("here");
        
        let chat: ChatMessage = serde_json::from_slice(&buf).unwrap();
        if chat.source_name == name {
            // 发送方
            let mut all = ALL.lock().await;
            let mut flag = false;
            for p in &mut all.all {
                if p.name == chat.target_name {
                    let tx = p.tx.clone();
                    let mut tx = tx.lock().await;                    
                    tx.write_u32(buf.len() as u32).await.unwrap();
                    tx.write(&mut buf).await.unwrap();
                    flag = true;
                }
            }

            if flag == false {
                for p in &mut all.all {
                    if p.name == chat.source_name {
                        let tx = p.tx.clone();
                        let mut tx = tx.lock().await;
                        let err = make_error_message(chat.source_name.clone(), "对方不在线");
                        let mut err = serde_json::to_string(&err).unwrap().into_bytes();
                        tx.write_u32(buf.len() as u32).await.unwrap();
                        tx.write(&mut err).await.unwrap();
                    }
                }
            }
        } else {
            println!("伪装的消息");
        }
    }
}


fn make_error_message(name: String, msg: &str) -> ChatMessage {
    ChatMessage { source_name: name.clone(), target_name: name, message: String::from(msg) }
}

