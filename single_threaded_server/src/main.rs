use std::{
    env,
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| "7878".to_string());
    let bind_address = format!("0.0.0.0:{}", port);
    
    let listener = TcpListener::bind(&bind_address).unwrap();
    println!("Server is listening on {}", bind_address);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established");

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "found.html")
    } else {
        ("HTTP/1.1 400 NOT FOUND", "notfound.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let lenght = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {lenght}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

