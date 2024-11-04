use std::io;

use tokio::{self, io::AsyncReadExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listner = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        match listner.accept().await {
            Ok((mut socket, addr)) => {
                let mut buf = [0; 1024];
                socket.read(&mut buf).await.unwrap();
                let message = String::from_utf8_lossy(&buf);
                println!("new client {:?}", addr);
                println!("message from the user :\n{:?}", message);
            }
            Err(e) => println!("error :{:?}", e),
        }
    }
}
