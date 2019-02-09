use std::env;
use std::io;
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle(mut stream: TcpStream) -> io::Result<u64> {
    // NOT: This might need set_{read,write}_timeout or async things... :'(
    stream
        .set_nonblocking(true)
        .expect("cannot set non-blocking mode");
    stream.set_nodelay(true).expect("cannot set nodelay mode");

    let bytes = match io::copy(&mut stream.try_clone().unwrap(), &mut stream) {
        Ok(b) => b,
        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => 0,
        Err(e) => {
            println!("wrn: {}", e);
            0
        },
    };
    Ok(bytes)
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
