use std::collections::HashMap;
use std::hash::Hash;
use std::io::{BufRead, BufReader, Read};
use std::str::SplitWhitespace;
use std::usize;
use std::{io::Write, net::TcpStream};
use std::net::TcpListener;
use regex::Regex;

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_result(&stream);
                println!("accepted new connection");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_result(stream: &TcpStream){
    let mut reader = 
        BufReader::new(stream);

    let mut req_line: String = String::new();
    let mut headers: String = String::new();
    let mut body: String = String::new();

    let mut http_method: String = String::new();
    let mut req_target: String = String::new();
    let mut http_version: String = String::new();

    //TODO Extract it for a specific method to get request_line
    reader.read_line(&mut req_line);
    for (index, item ) in req_line.split_whitespace().enumerate(){
        match index {
            0 => http_method = item.to_string(),
            1 => req_target = item.to_string(),
            2 => http_version = item.to_string(),
            _ => {} 
        }
    }

    //TODO Read headers and body
    reader.read_line(&mut headers);
    reader.read_line(&mut body);

    
    write_result(stream, b"HTTP/1.1 200 OK\r\n\r\n");
}


// fn handle_result(stream: TcpStream){
//     let tmp_stream = stream.try_clone().unwrap();

//     let buff = 
//         BufReader::new(stream).lines().next().unwrap();

//     match buff {
//         Ok(str) =>{
//             println!("req {}",str);
//             let mut header_info: SplitWhitespace = str.split_whitespace();
//             header_info.next();
//             let request_path: &str = header_info.next().unwrap();
//             let echo_path: Regex = Regex::new(r"^/echo/\w+$").unwrap();

//             match request_path {
//                 "/user-agent" => {
//                     //TODO Refactor above logic to made a filter or something like it to find the route
//                     write_result(tmp_stream, b"HTTP/1.1 200 OK\r\n\r\n");
//                 }
//                 rp if echo_path.is_match(rp) => {
//                     let param = &rp[6..rp.chars().count()];
//                     write_result(tmp_stream, format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
//                         param.chars().count(),param).as_bytes());
//                 }
//                 "/" => {
//                     write_result(tmp_stream, b"HTTP/1.1 200 OK\r\n\r\n");
//                 }
//                 _ => {
//                     write_result(tmp_stream, b"HTTP/1.1 404 Not Found\r\n\r\n");
//                 }
//             }
//             println!("My URL Path: {:?}", str);
            
//         }
//         Err(err ) => {
//             println!("Error {}", err);
//         }
//     }

// }

fn write_result(mut stream: &TcpStream, string_buffer: &[u8]){
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
