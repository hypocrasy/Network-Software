use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:34254")?;
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