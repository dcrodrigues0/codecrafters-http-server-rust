use core::str;
use std::collections::HashMap;
use std::fs::{read_dir, read_to_string, DirEntry, File};
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
    //TODO Implement a way that don't use .clone
    let buf_tmp = buf.clone();
    
    let mut req_lines: SplitWhitespace<'_> = buf.split_whitespace();
    let req_method: &str = match req_lines.nth(0) {
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
    
    let headers: HashMap<String, String> = 
        parse_header(buf_tmp.as_ref());

    print!("Headers: {:?}",headers);

    //TODO Organize the splitted words order
    let req_body = headers.iter()
        .filter(|(key, _value)| !is_header(key))
        .map(|(key, value )| format!("{} {} ", key, value))
        .collect::<String>();

    if req_method == "GET"{
        match req_target {
            "/" => {write_result(stream, b"HTTP/1.1 200 OK\r\n\r\n"); return},
            "/user-agent" => {exec_user_agent(stream, headers); return;},
            tg if tg.contains("/files") && !get_parameter(tg,String::from("files")).is_empty() => 
                {get_exec_files(stream, get_parameter(tg,String::from("files"))); return;},
            tg if tg.contains("/echo/") && !get_parameter(tg,String::from("echo")).is_empty() => 
                {exec_echo(stream, get_parameter(req_target, String::from("echo")), headers); return;},
            _ => {write_result(stream, b"HTTP/1.1 404 Not Found\r\n\r\n"); return;}
        }
    }
    
    if req_method == "POST"{
        match req_target {    
            tg if tg.contains("/files") && !get_parameter(tg,String::from("files")).is_empty() => 
                post_exec_files(stream, req_target, req_body.as_ref()),
            _ => write_result(stream, b"HTTP/1.1 404 Not Found\r\n\r\n")
        }
    }

}

fn post_exec_files(stream: TcpStream, target: &str, body: &str){
    let file_name = get_parameter(target, String::from("files")).into_iter().collect::<String>();
    if let Ok(mut file) = File::create(format!("/tmp/{}", file_name)){
        match file.write_all(body.as_bytes()) {
            Ok(_) => {
                write_result(stream, b"HTTP/1.1 201 Created\r\n\r\n");
                print!("File writed!");
            },
            Err(e) => {
                write_result(stream, b"HTTP/1.1 409 Conflict\r\n\r\n");
                print!("File not writed! {:?}",e)
            },
        };
    }
}

fn is_header(request_info: &str) -> bool{
    match request_info {
        "Host" => true,
        "User-Agent" => true,
        "Accept" => true,
        "Content-Type" => true,
        "Content-Length" => true,
        "Accept-Encoding" => true,
        _ => false   
    }
}

fn parse_header(req_line: &str) -> HashMap<String,String>{
    let mut headers: HashMap<String, String> = HashMap::new();
    let mut lines = req_line.lines();

    if let Some(_) = lines.next() {}

    for line in lines{
        if line.trim().is_empty(){
            break;
        }
    
        if let Some((key,value)) = line.split_once(':'){
            if is_header(key){
                headers.insert(
                    key.trim().to_string(),
                 value.trim().to_string());
            }
        }
    }

    headers
}

fn get_exec_files(stream: TcpStream, params: Vec<&str>){
    if let Ok(entries) = read_dir("/tmp/"){
        for entry in entries{
            if let Ok(entry) = entry{
                if entry.file_name().to_string_lossy() == params[0].to_string(){
                    if let Ok(metadata) = entry.metadata(){
                        write_result(stream, 
                            format!("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}",
                            metadata.len(), get_file_content(entry)).as_bytes());
                        return;
                    }   
                }
            }
        }
    }
    write_result(stream, b"HTTP/1.1 404 Not Found\r\n\r\n");
}

fn get_file_content(dir: DirEntry) -> String{
    match read_to_string(dir.path()) {
        Ok(file_content) => file_content,
        Err(_) => String::new(),  
    }
}

fn exec_user_agent(stream: TcpStream, headers: HashMap<String, String>){
    let or= &String::new();
    let user_agent = headers.get("User-Agent:").unwrap_or(or);
    write_result(stream, format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
        user_agent.chars().count(), user_agent).as_bytes());
}

fn exec_echo(stream: TcpStream, params: Vec<&str>, headers: HashMap<String, String>){
    let param: String = params[0].to_string();
    if headers.contains_key("Accept-Encoding:"){
        write_result(stream, format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nContent-Encoding: {}\r\nContent-Length: {}\r\n\r\n{}",
        headers.get("Accept-Encoding:").unwrap(), param.chars().count(), param).as_bytes());
        return;
    }
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
