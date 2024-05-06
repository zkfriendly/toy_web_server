use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread::{self, sleep},
    time::Duration,
};

use web_server::ThreadPool;

fn main() {
    let addr = "127.0.0.1:7878";
    let listener = TcpListener::bind(addr).unwrap();

    let tp = ThreadPool::new(4);

    for stream in listener.incoming() {
        ThreadPool::execute(|| handle_stream(stream.unwrap()));
    }
}

fn handle_stream(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, content) = match request_line.as_str() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /sleep HTTP/1.1" => {
            sleep(Duration::from_secs(10));
            ("HTTP/1.1 200 OK", "index.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let content = fs::read_to_string(content).unwrap();
    let content_len = content.len();
    let response = format!("{status_line}\r\nContent-Length: {content_len}\r\n\r\n{content}");
    stream.write_all(response.as_bytes()).unwrap()
}
