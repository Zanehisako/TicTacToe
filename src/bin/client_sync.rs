use std::{
    io::{Read, Write},
    net::TcpStream,
    time::Instant,
};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    loop {
        let mut buf = [0; 1024];
        println!("Enter your message :");
        let mut message = String::new();
        std::io::stdin().read_line(&mut message).unwrap();
        let before = Instant::now();
        stream.write_all(message.as_bytes()).unwrap();
        stream.read(&mut buf).unwrap();
        println!("time elapsed : {:?}", before.elapsed());
        let recived_message =
            String::from_utf8_lossy(&buf[..buf.iter().position(|&x| x == 0 || x == 10).unwrap()]);
        println!("the recived message is : {:?}", recived_message)
    }
}
