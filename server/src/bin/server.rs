use std::error::Error;
use tokio::{
    io::{AsyncWriteExt, BufWriter},
    net::{TcpListener, UdpSocket},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let socket = UdpSocket::bind("0.0.0.0:34254").await?;
    tokio::spawn(async move {
        let mut buf = [0; 50];
        loop {
            match socket.recv_from(&mut buf).await {
                Ok((_, src)) => {
                    println!("{:?}", std::str::from_utf8(&buf));

                    if let Err(e) = socket.send_to(&buf, src).await {
                        dbg!(e);
                        continue;
                    }
                    buf.fill(0);
                }
                Err(e) => {
                    dbg!(e);
                }
            }

            // if let Ok("ping") = std::str::from_utf8(&buf) {
            //     // println!("{}", "ping");
            //     socket.send_to("pong".as_bytes(), src)?;
            // }
        }
    });

    let speedso = TcpListener::bind("0.0.0.0:34254").await?;

    loop {
        let (socket, _addr) = speedso.accept().await?;

        tokio::spawn(async move {
            // Process each socket concurrently.
            println!("开始测速");
            match speed_test(socket).await {
                Ok(_ok) => {
                    // todo 后续逻辑
                }
                Err(e) => {
                    dbg!(e);
                    return;
                }
            }
        });
    }
}

async fn speed_test(socket: tokio::net::TcpStream) -> Result<(), Box<dyn Error>> {
    let (_rx, tx) = socket.into_split();
    let mut tx = BufWriter::with_capacity(1024*1024, tx);
    

    let buf = [0u8; 1024];
    // 测量下载网速
    for _i in 1..(1024*1024) {
        tx.write(&buf).await?;
    }

    tx.flush().await?;

    tx.write_i32(1919810).await?;
    tx.flush().await?;
    println!("测速结束");
    Ok(())
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
