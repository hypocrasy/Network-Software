use std::{error::Error,};
use tokio::{
    io::{AsyncWriteExt, AsyncReadExt},
    net::{TcpStream}, time,
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
    println!("ok");

    let mut handles = vec![];
    for _ in 0..100 {
        let handle = tokio::spawn(async move {
            let mut client = TcpStream::connect("127.0.0.1:34255").await.unwrap();
            let mut buf = vec![0x3fu8; 65535];
            let mut recvbuf = vec![0u8; 65535];
            let t0 = time::Instant::now();
            let wlen = client.write(&mut buf).await.unwrap();
            let rlen = client.read(&mut recvbuf).await.unwrap();
            let t1 = time::Instant::now();
            assert_eq!(wlen, rlen);
            assert_eq!(wlen, 65535);
            for i in 0..65535{
                assert_eq!(recvbuf[i], buf[i]);
            }
            let dur = t1 - t0;
            println!("{:?}", dur);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.await.unwrap();
    }

    Ok(())
    // loop {
    //     let (mut socket, _addr) = speedso.accept().await?;
    //     let mut buf = [0u8; 65535];

    //     tokio::spawn(async move {
    //         loop {
    //             let n = socket.read(&mut buf).await.unwrap();
    //             socket.write(&buf[0..n]).await.unwrap();
    //         }
    //     });
    // }
}