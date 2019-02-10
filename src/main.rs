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
        },
    };

    let mut body = false;
    for line in buf.lines() {
        if line == "" {
            body = true;
        } else if body {
            stream
                .write_all(format!("{}\r\n", line).as_bytes())
                .unwrap();
        }
    }
}

fn get_addr() -> String {
    let host = match env::var("HOST") {
        Ok(value) => value,
        Err(_) => "127.0.0.1".to_string(),
    };
    let port = match env::var("PORT") {
        Ok(value) => value,
        Err(_) => "8080".to_string(),
    };
    format!("{}:{}", host, port)
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind(get_addr())?;

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                thread::spawn(move || handle(s));
            },
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
        assert_eq!("127.0.0.1:8080", addr);
    }
}
