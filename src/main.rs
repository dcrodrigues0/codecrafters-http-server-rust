use std::collections::HashMap;
use std::io::Read;
use std::iter::Peekable;
use std::str::SplitWhitespace;
use std::time::Duration;
use std::{io::Write, net::TcpStream};
use std::net::TcpListener;

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                stream.set_read_timeout(Some(Duration::new(0, 1000)))
                    .expect("set_read_timeout call failed");
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
    let mut buf:String = String::new();
    match stream.read_to_string(&mut buf) {
        Ok(_) => return,
        Err(e) => println!("Stream read error: {:?}",e)
    }

    let mut req_lines: SplitWhitespace<'_> = buf.split_whitespace();
    let req_method: &str = match req_lines.nth(0) {
        Some(line) => line,
        None => return
    };
    let req_target: &str = match req_lines.nth(0) {
        Some(line) => line,
        None => return
    };
    let http_version: &str = match req_lines.nth(0) {
        Some(line) => line,
        None => return
    };
    
    let mut peek_req_lines: Peekable<_> = req_lines.peekable();
    let mut headers: HashMap<&str, &str> = HashMap::new();
    while peek_req_lines.peek().is_some() {       
        headers.insert(peek_req_lines.next().unwrap_or(""),
         peek_req_lines.next().unwrap_or(""));
    }
    
    match req_target {
        target if target.contains("/echo") && has_parameters(target) => write_result(stream, b"HTTP/1.1 200 OK\r\n\r\n"),
        "/" => write_result(stream, b"HTTP/1.1 200 OK\r\n\r\n"),
        _ => write_result(stream, b"HTTP/1.1 404 Not Found\r\n\r\n")
    }
    
    // write_result(stream, b"HTTP/1.1 200 OK\r\n\r\n");
}

fn has_parameters(req_target:&str ) -> bool{
    //TODO Think in a good way to rescue parameter values
    req_target.split('/');
    true
}

fn write_result(mut stream: TcpStream, string_buffer: &[u8]){
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
