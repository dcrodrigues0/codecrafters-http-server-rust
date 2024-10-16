use std::io::{BufRead, BufReader};
use std::{io::Write, net::TcpStream};
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

fn handle_result(stream: TcpStream){
    let tmp_stream = stream.try_clone().unwrap();

    let buff = 
        BufReader::new(stream).lines().next().unwrap();

    match buff {
        Ok(str) =>{
            let mut header_info = str.split_whitespace();
            header_info.next();
            let request_path = header_info.next().unwrap();

            match request_path {
                "/" => {
                    write_result(tmp_stream, b"HTTP/1.1 200 OK\r\n\r\n");
                }
                _ => {
                    write_result(tmp_stream, b"HTTP/1.1 404 Not Found\r\n\r\n");
                }
            }
            println!("My URL Path: {:?}", str);
            
        }
        Err(err ) => {
            println!("Error {}", err);
        }
    }

}

fn write_result(mut stream:TcpStream, string_buffer: &[u8]){
    let write_result 
        = stream.write(string_buffer);
        
    match write_result {
        Ok(_stream ) =>{
            println!("Message sent successfully");
        }
        Err(err) =>{
            println!("Cannot send message {}", err);
        }
    }
}
