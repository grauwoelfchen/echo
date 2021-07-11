use std::env;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle(mut stream: TcpStream) {
    // NOTE:
    // This might need set_{read,write}_timeout or async things... :'(
    stream
        .set_nonblocking(true)
        .expect("cannot set non-blocking mode");
    stream.set_nodelay(true).expect("cannot set nodelay mode");

    // NOTE:
    // Previously, we implemented this using just `io::copy()`. However it
    // returns also sent header. In here, only body should be returned.
    //
    // And apparently, it seems that `read_to_end()` or BufReader's `lines()`
    // returns by EOF between header and body. Is there any good way? :'(

    let mut buf = String::new();
    let _size = match stream.read_to_string(&mut buf) {
        Ok(s) => s,
        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => 0,
        Err(e) => {
            panic!("err: {}", e);
        }
    };

    let mut has_body = false;
    for line in buf.lines() {
        if line.is_empty() {
            has_body = true;
        } else if has_body {
            let response = format!("HTTP/1.1 200 OK\r\n\r\n{}\r\n", line);
            stream.write_all(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
}

fn get_addr() -> String {
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    format!("{}:{}", host, port)
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind(get_addr())?;

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                thread::spawn(move || handle(s));
            }
            Err(e) => panic!("err: {}", e),
        }
    }
    Ok(())
}

#[cfg(test)]
mod main {
    use super::*;

    #[test]
    fn test_get_addr() {
        let addr = get_addr();
        assert_eq!("0.0.0.0:8000", addr);
    }
}
