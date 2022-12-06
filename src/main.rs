use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use non_library_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                pool.execute(|| {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                print!("connection failed, {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let request_string = request_line.split('/').collect::<Vec<&str>>(); //::Vec<&str>;
    println!("{:#?}", request_string);

    let (status_line, filename) = match request_string.len() {
        3 => match request_string[1] {
            "sleep HTTP" => {
                thread::sleep(Duration::from_secs(5));
                ("HTTP/1.1 200 OK", "hello.html")
            }
            " HTTP" => ("HTTP/1.1 200 OK", "hello.html"),
            _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
        },
        4 => {
            if request_string[1] == "sleep" {
                let time = request_string[2]
                    .chars()
                    .next()
                    .unwrap()
                    .to_digit(10)
                    .unwrap();
                thread::sleep(Duration::from_secs(time.into()));
                ("HTTP/1.1 200 OK", "hello.html")
            } else {
                ("HTTP/1.1 404 NOT FOUND", "404.html")
            }
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
