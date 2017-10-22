mod parser;

use std::net::TcpListener;
use std::thread;
use std::io::{Read, Write};
use std::io;

fn server_start() -> io::Result<()> {
    let lis = TcpListener::bind("127.0.0.1:8080")?;

    for stream in lis.incoming() {
        let mut stream = match stream {
            Ok(stream) => stream,
            Err(e) => {
                println!("An error occurred while accepting a connection: {}", e);
                continue;
            }
        };

        let _ = thread::spawn(
            move || -> io::Result<()> {
                loop {
                    let mut b = [0; 1024];
                    let n = stream.read(&mut b)?;
                    if n == 0 {
                        return Ok(());
                    } else {
                        stream.write(&b[0..n])?;
                    }
                }});
    }
    Ok(())
}

fn main() {
    match server_start() {
        Ok(_) => (),
        Err(e) => println!("{:?}", e),
    }
}
