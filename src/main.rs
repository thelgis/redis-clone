mod resp;
mod resp_result;

use crate::resp::RESP;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?; // defines the function's return type

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                tokio::spawn(handle_connection(stream)); // spawn a new task for each new connection 
            }
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        }
    }
}

async fn handle_connection(mut stream: TcpStream) {
    println!("Incoming connection from: {}", stream.peer_addr().unwrap());

    let mut buffer: [u8; 512] = [0; 512]; // u8 used to represent one Byte

    loop {
        match stream.read(&mut buffer).await {
            Ok(size) if size != 0 => {
                let response = RESP::SimpleString(String::from("PONG"));

                if let Err(e) = stream.write_all(response.to_string().as_bytes()).await {
                    eprintln!("Error writing to socket: {}", e);
                }
            }
            Ok(_) => {
                println!("Connection closed: {}", stream.peer_addr().unwrap());
                break;
            }
            Err(error) => {
                println!("Error when reading from stream: {}", error);
                break;
            }
        }
    }
}
