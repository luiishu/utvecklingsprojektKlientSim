#[allow(unused)]
use std::{
    fs,
    io::{self, prelude::*, BufReader},
    net::{IpAddr, Ipv4Addr,SocketAddr,  TcpListener, TcpStream},
    process,
    time,
    thread,
};

#[allow(unused)]
fn main() {
    //let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    test();

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 7878);
    //let mut stream = TcpStream::connect_timeout("127.0.0.1:7878", Duration::new(5, 0)).unwrap();
    let mut stream = TcpStream::connect(&addr).unwrap();

    let mut sending_msg = "Client: Hello from Client!";
    let mut receiving_msg = String::new();

    stream.write(sending_msg.as_bytes()).unwrap();

    let mut buffer = [0u8; 2048];

    let bytes_read = stream.read(&mut buffer).unwrap();
    let receiving_msg = String::from_utf8_lossy(&buffer[..]).to_string();

    println!("Bytes read from from stream: {}", bytes_read);
    println!("Message received:\n{}", receiving_msg);
}

#[allow(unused)]
fn test() {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 7878);
    let sleep_time = time::Duration::from_millis(5000);

    let mut counter = 0;

    let mut sending_msg = "Client: Hello from Client!";
    let mut receiving_msg = String::new();

    loop {
        counter += 1;
        println!("Trying to connect to server with counter {counter}...",);
        let mut stream = TcpStream::connect(&addr).unwrap();
        let mut sending_msg = "Client: Hello from Client!";
        let mut receiving_msg = String::new();

        stream.write(sending_msg.as_bytes()).unwrap();

        let mut buffer = [0u8; 2048];

        let bytes_read = stream.read(&mut buffer).unwrap();
        let receiving_msg = String::from_utf8_lossy(&buffer[..]).to_string();

        println!("Bytes read from from stream: {}", bytes_read);
        println!("Message received:\n{}", receiving_msg);
        thread::sleep(sleep_time);
    }

}
