#[cfg(test)]
#[macro_use]
extern crate lazy_static;

#[cfg(test)]
extern crate parking_lot;

use std::env;
use std::io::{self, BufReader, Write};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;

fn get_local_addr() -> String {
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    format!("{}:{}", host, port)
}

fn handle(stream: TcpStream) {
    stream.set_nodelay(true).expect("cannot set nodelay mode");

    // nonblocking
    stream
        .set_nonblocking(true)
        .expect("cannot set non-blocking mode");
    // with blocking
    // stream.set_read_timeout(Some(Duration::from_millis(1000))).unwrap();
    // stream.set_write_timeout(Some(Duration::from_millis(1000))).unwrap();

    let (reader, writer) = &mut (&stream, &stream);

    let mut buf = BufReader::new(reader);

    // e.g. POST / HTTP/1.1\r\n
    let mut request_line = String::new();
    let _num_bytes = buf.read_line(&mut request_line);
    println!("{}", request_line);

    // https://doc.rust-lang.org/std/io/enum.ErrorKind.html#variant.WouldBlock
    //
    // WouldBlock:
    //   The operation needs to block to complete, but the blocking operation
    //   was requested to not occur.
    //
    // Note:
    // Ignore the blocking error at here for now.
    let mut input = String::new();
    let _ = match buf.read_to_string(&mut input) {
        Ok(s) => s,
        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => 0,
        Err(e) => {
            panic!("err: {}", e);
        }
    };

    let mut headers = Vec::<&str>::new();
    let mut body = String::new();

    let mut is_body = false;
    for line in input.lines() {
        if line.is_empty() {
            is_body = true;
            continue;
        }
        if !is_body {
            headers.push(line);
        } else {
            body.push_str(line);
        }
    }
    println!("{}\n", headers.join("\n"));
    println!("{}", body);

    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}\r\n", body);
    writer.write_all(response.as_bytes()).unwrap();
    writer.flush().unwrap();
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind(get_local_addr())?;

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
mod test {
    use super::*;

    use std::collections::HashMap;
    use std::panic::{self, AssertUnwindSafe, UnwindSafe};

    use parking_lot::Mutex;

    // e.g. hashmap! { "key" => "value" }
    #[macro_export]
    macro_rules! hashmap(
        { $($key:expr => $value:expr),+ } => {
            {
                let mut m = HashMap::new();
                $(m.insert($key, $value);)+
                m
            }
        };
    );

    fn with<T>(
        keys: &'static str,
        vars: HashMap<&'static str, &'static str>,
        test: T,
    ) where
        T: FnOnce() + UnwindSafe,
    {
        lazy_static! {
            static ref ENV_LOCK: Mutex<()> = Mutex::new(());
        }
        let _lock = ENV_LOCK.lock();

        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            let mut origin: HashMap<&str, Result<String, env::VarError>> =
                HashMap::new();

            for key in keys.split('\n') {
                if key.is_empty() {
                    continue;
                }

                origin.insert(key, env::var(key));

                if !vars.contains_key(key) {
                    env::remove_var(key);
                } else {
                    env::set_var(key, vars.get(key).unwrap());
                }
            }

            let result = test();

            for (key, value) in origin.iter() {
                match value {
                    Ok(v) => env::set_var(key, v),
                    Err(_) => env::remove_var(key),
                }
            }

            result
        }));
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_local_addr_in_default() {
        let vars: HashMap<&'static str, &'static str> = HashMap::new();

        with(
            r#"
HOST
PORT
"#,
            vars,
            || {
                let addr = get_local_addr();
                assert_eq!(addr, "0.0.0.0:3000");
            },
        );
    }

    #[test]
    fn test_get_local_addr_host_is_set() {
        let vars: HashMap<&'static str, &'static str> = hashmap! {
            "HOST" => "127.0.0.1"
        };

        with(
            r#"
HOST
PORT
"#,
            vars,
            || {
                let addr = get_local_addr();
                assert_eq!(addr, "127.0.0.1:3000");
            },
        );
    }

    #[test]
    fn test_get_local_addr_port_is_set() {
        let vars: HashMap<&'static str, &'static str> = hashmap! {
            "PORT" => "5000"
        };

        with(
            r#"
HOST
PORT
"#,
            vars,
            || {
                let addr = get_local_addr();
                assert_eq!(addr, "0.0.0.0:5000");
            },
        );
    }

    #[test]
    fn test_get_local_addr_is_set() {
        let vars: HashMap<&'static str, &'static str> = hashmap! {
            "HOST" => "localhost",
            "PORT" => "5000"
        };

        with(
            r#"
HOST
PORT
"#,
            vars,
            || {
                let addr = get_local_addr();
                assert_eq!(addr, "localhost:5000");
            },
        );
    }
}
