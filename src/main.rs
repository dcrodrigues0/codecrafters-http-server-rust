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
    //TODO Understand why always entering in Err
    match stream.read_to_string(&mut buf) {
        Ok(_) => return,
        Err(e) => println!("Stream read error: {:?}",e)
    }

    let mut req_lines: SplitWhitespace<'_> = buf.split_whitespace();
    let _req_method: &str = match req_lines.nth(0) {
        Some(line) => line,
        None => return
    };
    let req_target: &str = match req_lines.nth(0) {
        Some(line) => line,
        None => return
    };
    let _http_version: &str = match req_lines.nth(0) {
        Some(line) => line,
        None => return
    };
    
    let mut peek_req_lines: Peekable<_> = req_lines.peekable();
    let mut headers: HashMap<String, String> = HashMap::new();
    while peek_req_lines.peek().is_some() {       
        headers.insert(peek_req_lines.next().unwrap_or("").to_string(),
         peek_req_lines.next().unwrap_or("").to_string());
    }
    
    match req_target {
        "/" => write_result(stream, b"HTTP/1.1 200 OK\r\n\r\n"),
        tg if tg.contains("/user-agent") => exec_user_agent(stream, headers),
        tg if tg.contains("/echo/") && !get_parameter(tg,String::from("echo")).is_empty() => 
            exec_echo(stream, get_parameter(req_target, String::from("echo"))),
        _ => write_result(stream, b"HTTP/1.1 404 Not Found\r\n\r\n")
    }
    
}

fn exec_user_agent(stream: TcpStream, headers: HashMap<String, String>){
    let or= &String::new();
    let user_agent = headers.get("User-Agent:").unwrap_or(or);
    write_result(stream, format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
        user_agent.chars().count(), user_agent).as_bytes());
}

fn exec_echo(stream: TcpStream, params: Vec<&str>){
    let param: String = params[0].to_string();
    write_result(stream, format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
        param.chars().count(), param).as_bytes());
}

fn get_parameter(req_target:&str, endpoint:String) -> Vec<&str>{
    req_target.split("/").filter(|str| !str.is_empty() && str.to_string() != endpoint)
    .collect::<Vec<&str>>()
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
