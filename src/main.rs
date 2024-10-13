use std::{io::Write, net::TcpStream};
#[allow(unused_imports)]
use std::net::TcpListener;

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_result(stream);
                println!("accepted new connection");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_result(mut stream: TcpStream){
    let write_result 
        = stream.write(b"HTTP/1.1 200 OK\r\n\r\n");
    
    match write_result {
        Ok(_stream ) =>{
            println!("Message sent successfully");
        }
        Err(e) =>{
            println!("Cannot send message {}", e);
        }
    }
}
