use tokio::{
    self,
    io::{self, AsyncReadExt, AsyncWriteExt},
};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listner = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, addr) = listner.accept().await.unwrap();
        socket.set_nodelay(true).unwrap();
        println!("new client connected : {:?}", addr);
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(n) if n == 0 => return, // Connection closed
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("Failed to read from socket: {}", e);
                        return;
                    }
                };

                // Process received data
                let message = String::from_utf8_lossy(&buf[..n - 1]);
                println!("Received: {:?} From:{:?}", message, addr);

                // Echo the message back to the client
                if let Err(e) = socket.write_all("Sm3tk ya ...".as_bytes()).await {
                    eprintln!("Failed to write to socket: {}", e);
                    return;
                }
            }
        });
    }
}
