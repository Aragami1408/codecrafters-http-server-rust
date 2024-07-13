use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");
    
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                handle_connection(stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let request_line: Vec<&str> = http_request[0].split(" ").collect();
    let mut response = String::from("");

    let path = request_line[1];
    if path == "/" {
        response = String::from("200 OK");
    }
    else if path.starts_with("/echo/") {
        let (_, parameter) = path.split_at(6);
        response = String::from(format!("200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", 
            parameter.len(), parameter));
    }
    else {
        response = String::from("404 Not Found");
    }

    stream.write_all(format!("HTTP/1.1 {}\r\n\r\n", response).as_bytes()).unwrap();
}
