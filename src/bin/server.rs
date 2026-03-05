use std::io::Write;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer: [u8; 1000] = [0; 1000];

    println!("OMG A ClIenT :O {}", stream.peer_addr()?);
    stream.read(&mut buffer)?;
    println!("Client says : {}", String::from_utf8_lossy(&buffer));
    stream.write(b"Hello World!")?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:9583")?;

    for stream in listener.incoming() {
        handle_client(stream?)?;
    }
    Ok(())
}
