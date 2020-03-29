use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::{fs, thread};
use std::time::Duration;

fn main() {
    let addr = "127.0.0.1:7878";
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, contents_path) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n", "./resources/hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(10));
        ("HTTP/1.1 200 OK\r\n", "./resources/hello.html")
    } else {
        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
        ("HTTP/1.1 404 NOT FOUND\r\n", "./resources/404.html")
    };

    let contents = fs::read_to_string(contents_path).unwrap();
    let content_len_header = contents_len(&contents);

    let response = format!("{}{}{}", status_line, content_len_header, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}

fn contents_len(contents: &String) -> String {
    return format!("Content-Length: {}\r\n\r\n", contents.len());
}