use std::env;
use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let message: String = args.join(" ");

    let mut stream = TcpStream::connect("127.0.0.1:9583")?;
    //let mut buffer: [u8; 12] = [0; 12];
    let mut str: String = Default::default();
    stream.write(message.as_bytes())?;

    stream.read_to_string(&mut str)?;
    println!("responce from server {}", str);
    Ok(())
}
