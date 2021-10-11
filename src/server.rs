use crate::http::Request;
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    m_addr: String,
}

impl Server {
    // associated functions are known as static functions in C++
    pub fn new(addr: String) -> Self {
        Server { m_addr: addr }
    }

    pub fn run(&self) {
        println!("Running server on {}", self.m_addr);
        let listener = TcpListener::bind(&self.m_addr).unwrap();
        // special syntax for while true
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!(
                                "Received a request from: {}",
                                String::from_utf8_lossy(&buffer)
                            );
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {}
                                Err(error) => {
                                    println!("Request failed: {}", error);
                                }
                            }
                        }
                        Err(error) => {
                            println!("Failed to read from connection: {}", error);
                        }
                    }
                }
                Err(error) => {
                    println!("Failed to establish a connection: {}", error);
                }
            }
        }
    }
}
