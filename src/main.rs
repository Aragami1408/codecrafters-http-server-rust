use std::{
    env, fs,
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
                std::thread::spawn(|| handle_connection(stream));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn get_headers<'a>(headers: &'a [String], header: &'a str) -> Option<&'a str> {
    for line in headers {
        let lowercase_line = line.to_lowercase();
        if lowercase_line.starts_with(&header.to_lowercase()) {
            let (_, result) = line.split_at(header.len() + 2);
            return Some(result);
        }
    }

    None
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("HTTP Request: {:?}", http_request);

    let request_line: Vec<&str> = http_request[0].split(" ").collect();

    let mut response = String::from("");

    let path = request_line[1];
    if path == "/" {
        response = String::from("200 OK");
    } else if path.starts_with("/echo/") {
        let (_, parameter) = path.split_at(6);
        response = String::from(format!(
            "200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            parameter.len(),
            parameter
        ));
    } else if path.starts_with("/user-agent") {
        let headers = &http_request[1..http_request.len()];
        if let Some(user_agent) = get_headers(headers, "User-Agent") {
            response = String::from(format!(
                "200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                user_agent.len(),
                user_agent
            ));
        } else {
            panic!("Can't find user agent");
        }
    } else if path.starts_with("/files/") {
        let (_, filename) = path.split_at(7);

        let args: Vec<String> = env::args().collect();
        if (args[1] != "--directory") {
            panic!("Please specify directory by using --directory option");
        }

        let mut dir = args[2].clone();
        let file = fs::read(dir + filename);

        match file {
            Ok(file_content) => {
                response = String::from(format!(
                    "200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}",
                    file_content.len(),
                    String::from_utf8(file_content).expect("file content")
                ));
            }
            Err(_) => response = String::from("404 Not Found"),
        }
    } else {
        response = String::from("404 Not Found");
    }

    stream
        .write_all(format!("HTTP/1.1 {}\r\n\r\n", response).as_bytes())
        .unwrap();
}
