use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:34254")?;
    let mut buf = [0; 50];
    loop {
        let (_, src) = socket.recv_from(&mut buf)?;
        println!("{:?}", std::str::from_utf8(&buf));
        
        socket.send_to(&buf, src)?;
        buf.fill(0);
        // if let Ok("ping") = std::str::from_utf8(&buf) {
        //     // println!("{}", "ping");
        //     socket.send_to("pong".as_bytes(), src)?;
        // }
    }
    // Ok(())
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