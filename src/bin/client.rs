use std::env;
use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let message: String = args.join(" ");

    let mut stream = TcpStream::connect("127.0.0.1:9583")?;
    //let mut buffer: [u8; 12] = [0; 12];
    let mut buffer2: Vec<u8> = vec![];
    stream.write(message.as_bytes())?;

    stream.read(&mut buffer2)?;
    println!("responce from server {}", String::from_utf8_lossy(&buffer2));
    Ok(())
}
