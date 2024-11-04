use tokio::{
    self,
    io::{AsyncReadExt, AsyncWriteExt},
};

#[tokio::main]
async fn main() {
    let mut stream = tokio::net::TcpStream::connect("127.0.0.1:8080")
        .await
        .unwrap();

    let message = "Hello";
    stream.write_all(message.as_bytes()).await.unwrap();
    let mut buf = [0; 1024];
    stream.read(&mut buf).await.unwrap();
    let recived_message = String::from_utf8_lossy(&buf);
    println!("the recived message is : {:?}", recived_message)
}
