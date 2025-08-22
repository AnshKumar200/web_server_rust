use std::{
    env,
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
    let request_line = match buf_reader.lines().next() {
        Some(Ok(line)) => line,
        Some(Err(_)) => {
            println!("Error reading request line");
            return;
        }
        None => {
            println!("No request line recieved.");
            return;
        }
    };

    let (status_line, contents) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", include_str!("found.html"))
    } else {
        ("HTTP/1.1 400 NOT FOUND", include_str!("notfound.html"))
    };

    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

