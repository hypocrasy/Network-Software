use std::error::Error;
use tokio::{
    io::{AsyncWriteExt, AsyncReadExt},
    net::{TcpListener},
};

fn main() -> Result<(), Box<dyn Error>> {
    tokio::runtime::Builder::new_current_thread().enable_all()
        .build().unwrap()
        .block_on(async {
            // async main
            async_main().await.unwrap()
        });

    Ok(())
}

async fn async_main() -> Result<(), Box<dyn Error>>{

    let speedso = TcpListener::bind("0.0.0.0:34255").await?;

    loop {
        let (mut socket, _addr) = speedso.accept().await?;
        let mut buf = vec![0u8; 65535];

        tokio::spawn(async move {
            loop {
                let n = socket.read(&mut buf).await.unwrap();
                socket.write(&buf[0..n]).await.unwrap();
            }
        });
    }
}
// use log::*;
// use tokio::net::UdpSocket;
// use std::sync::{Arc};
// use tokio::sync::Mutex;

// #[tokio::main]
// async fn main() -> std::io::Result<()> {
//     let socket = Arc::new(Mutex::new(UdpSocket::bind("127.0.0.1:34254").await?));
//     loop {
//         let mut buf = [0; 50];

//         let so = socket.lock().await;
//         let so1 = socket.clone();
//         let (_, src) = so.recv_from(&mut buf).await?;

//         tokio::spawn(async move {

//             let so = so1.lock().await;
//             println!("{:?}", std::str::from_utf8(&buf));

//             so.send_to(&buf, src).await.unwrap();
//         });

//         // if let Ok("ping") = std::str::from_utf8(&buf) {
//         //     // println!("{}", "ping");
//         //     socket.send_to("pong".as_bytes(), src)?;
//         // }
//     }
//     // Ok(())
// }
