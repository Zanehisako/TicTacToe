use std::{
    collections::HashSet,
    io::{Read, Write},
    net::TcpListener,
};

fn calculate_surface(i: i32, j: i32, k: i32) -> i32 {
    2 * (i * j) + 2 * (i * k) + 2 * (j * k)
}

fn calculate_surface_max_min(n: i32) -> String {
    let mut combinations = HashSet::new();

    for i in 1..=n {
        let mut j = 1;
        while j * i <= n {
            let k = (n / (i * j)) as i32;
            if i * j * k == n {
                let surface = calculate_surface(i, j, k);
                combinations.insert(surface);
            }
            j += 1;
        }
    }

    let max_surface = combinations.iter().max().unwrap_or(&0);
    let min_surface = combinations.iter().min().unwrap_or(&0);

    format!(
        "Minimum surface area: {}\nMaximum surface area: {}",
        min_surface, max_surface
    )
}

fn main() {
    let listner = TcpListener::bind("127.0.0.1:8080").unwrap();
    loop {
        let mut buf = [0; 1024];
        let (mut socket, addr) = listner.accept().unwrap();
        println!("client {:?}", addr);
        let message = socket.read(&mut buf);
        println!("{:?}", message);
        let result = calculate_surface_max_min(10000000);
        socket.write(&result.as_bytes()).unwrap();
        println!("the min is : {:?}", result);
    }
}
