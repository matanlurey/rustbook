use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    process::exit,
    thread,
    time::Duration,
};

use lib::ThreadPool;

mod lib;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap_or_else(|e| {
        eprintln!("Could not listen: {:?}", e);
        exit(1);
    });
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle(stream);
        });
    }

    println!("Shutting down.");
}

struct Response {
    status: String,
    contents: String,
}

impl Response {
    fn create_ok(contents: String) -> Response {
        Response {
            status: String::from("HTTP/1.1 200 OK"),
            contents,
        }
    }

    fn create_not_found(contents: String) -> Response {
        Response {
            status: String::from("HTTP/1.1 400 NOT FOUND"),
            contents,
        }
    }
}

fn handle(mut stream: TcpStream) {
    let request: Vec<_> = BufReader::new(&mut stream)
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let response = match &request.first().unwrap()[..] {
        "GET / HTTP/1.1" => {
            let contents = fs::read_to_string("chapter/20-web-server/index.html").unwrap();
            Response::create_ok(contents)
        }
        "GET /sleep HTTP/1.1" => {
            // Simulate a slower computation.
            thread::sleep(Duration::from_secs(2));
            let contents = fs::read_to_string("chapter/20-web-server/index.html").unwrap();
            Response::create_ok(contents)
        }
        _ => {
            let contents = fs::read_to_string("chapter/20-web-server/404.html").unwrap();
            Response::create_not_found(contents)
        }
    };

    let response = format!(
        "{}\r\nContent-length: {}\r\n\r\n{}",
        response.status,
        response.contents.len(),
        response.contents
    );
    stream.write_all(response.as_bytes()).unwrap();
}
