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
    let mut buffer = [0; 1024];

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                // Connection closed by the client
                break;
            }
            Ok(n) => {
                if buffer[n - 1] == 10 {
                    let message = String::from_utf8_lossy(&buffer[111..n]);
                    println!("Received message: {}", message);
                    // Do whatever you want with the received message
                }
            }
            Err(err) => {
                eprintln!("Error reading from socket: {}", err);
                break;
            }
        }
    }
}
