use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap(); // unwrapping for simplicity in toy examples

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                handle_connection(&mut stream);
            }
            Err(error) => {
                println!("Error when accepting connection: {}", error);
            }
        }
    }
}

fn handle_connection(stream: &mut TcpStream) {
    let mut buffer: [u8; 512] = [0; 512]; // u8 used to represent one Byte

    loop {
        match stream.read(&mut buffer) {
            Ok(size) if size != 0 => {
                let response = "+PONG\r\n";

                stream.write(response.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
            Ok(_) => {
                println!("Connection closed!");
                break;
            }
            Err(error) => {
                println!("Error when reading from stream: {}", error);
                break;
            }
        }
    }
}
