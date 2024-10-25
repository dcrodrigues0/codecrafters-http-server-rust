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
    let reader = 
        BufReader::new(stream);

    let mut http_method: String = "".to_string();
    let mut request_target: String = "".to_string();
    let mut http_version: String = "".to_string();
    
    let mut req_props: Vec<String> = Vec::new();
    for (i,line) in reader.lines().enumerate(){
        let req_line: String = line.unwrap();

        if i == 0 {    
            let mut req_info: SplitWhitespace<'_> = req_line.split_whitespace();
            http_method = req_info.next().unwrap().to_string();
            request_target = req_info.next().unwrap().to_string();
            http_version = req_info.next().unwrap().to_string();
        }else{
            let headers = req_line.split_whitespace();
            for header in headers {
                if !header.is_empty() && header.to_string() != ""{
                    req_props.push(header.to_string());
                }
            }
        }
    }

    println!("teste {:?}", req_props);
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
