use std::{net::TcpStream, io::{Write, Read}, str::from_utf8};

fn main() {
    let mut stream = TcpStream::connect("localhost:3000").unwrap();

    stream.write("hello tcpclient!".as_bytes()).unwrap();

    let mut buffer = [0;15];
    stream.read(&mut buffer).unwrap();

    println!(
        "Response from server:{:?}",
        from_utf8(&buffer)
    )
}
