use std::io::{self, Write};

use std::{
    io::Read,
    net::{TcpListener, TcpStream},
    thread,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4200").expect("Failed to bind address");

    println!("Server listening on port 4200...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Spawn a new thread for each incoming connection
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(err) => {
                eprintln!("Error accepting connection: {}", err);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer_a: [u8; 1024] = [0; 1024];
    let mut buffer_b: [u8; 1024] = [0; 1024];

    let mut buffer: &mut [u8; 1024] = &mut buffer_a;
    let mut prev_buffer: &mut [u8; 1024] = &mut buffer_b;

    let mut prev_n: usize = 0;

    loop {
        match stream.read(buffer) {
            Ok(0) => {
                // Connection closed by the client
                break;
            }
            Ok(n) => {
                if buffer[0] == 0x1b {
                    let tmp_buffer = buffer;
                    buffer = prev_buffer;
                    prev_buffer = tmp_buffer;
                    prev_n = n;
                } else if prev_buffer[0] == 0x1b {
                    let message = String::from_utf8_lossy(&prev_buffer[111..prev_n]);
                    print!("{}", message);
                    // Do whatever you want with the received message
                } else if prev_buffer[0] != 0x0 {
                    let message = String::from_utf8_lossy(&prev_buffer[0..prev_n]);
                    print!("{}", message);
                    // Do whatever you want with the received message
                }

                io::stdout().flush().unwrap();
            }
            Err(err) => {
                eprintln!("Error reading from socket: {}", err);
                break;
            }
        }
    }
}
