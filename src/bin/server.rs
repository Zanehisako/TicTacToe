use std::{
    io::{Read, Write},
    net::TcpListener,
};

fn calculate_surface(i: u32, j: u32, k: u32) -> u32 {
    return 2 * ((i * j) + (i * k) + (j * k));
}

fn calcualte_surface_max_min() -> (u32, u32) {
    print!("starting the function");
    let mut combinations = [0; 1000];
    for i in 1..1000 {
        print!("i: {:?}", i);
        let mut j = 1;
        loop {
            let k = 1000 / (i * j);
            if (i * j * k) == 1000 {
                let surface = calculate_surface(i, j, k);
                combinations[i as usize] = surface;
                if j > 1000 - 1 - i {
                    break;
                } else {
                    j += 1;
                }
            }
        }
    }
    return (
        *combinations.iter().min().unwrap(),
        *combinations.iter().max().unwrap(),
    );
}
fn main() {
    let listner = TcpListener::bind("127.0.0.1:8080").unwrap();
    let (min, _) = calcualte_surface_max_min();
    println!("the min is : {:?}", min);
    loop {
        let mut buf = [0; 1024];
        let (mut socket, addr) = listner.accept().unwrap();
        println!("client {:?}", addr);
        let message = socket.read(&mut buf);
        println!("{:?}", message);
        let (min, _) = calcualte_surface_max_min();
        println!("the min is : {:?}", min);
        socket.write(&min.to_le_bytes()).unwrap();
    }
}
