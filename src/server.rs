use crate::http::Request;
use std::convert::TryFrom;
use std::io::Read;
use std::net::{TcpListener, TcpStream};

pub struct Server {
    m_addr: String,
    m_buff_size: usize,
}

impl Server {
    // associated functions are known as static functions in C++
    pub fn new(addr: String) -> Self {
        Server {
            m_addr: addr,
            m_buff_size: 1024,
        }
    }

    fn read_from_tcpstream(&self, stream: &mut TcpStream) {
        let mut buffer: Vec<u8> = vec![0; self.m_buff_size];
        match stream.read(&mut buffer) {
            Ok(_) => {
                println!(
                    "Received a request from: {}",
                    String::from_utf8_lossy(&buffer)
                );
                // try to convert 
                match Request::try_from(&buffer[..]) {
                    Ok(req) => {
                        println!("{:#?}", req);
                    }
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

    pub fn run(&self) {
        println!("\nRunning server on {}", self.m_addr);
        let listener = TcpListener::bind(&self.m_addr).unwrap();
        loop {
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    println!("Connection established: {:#?}", addr);
                    self.read_from_tcpstream(&mut stream);
                }
                Err(error) => {
                    println!("Failed to establish a connection: {}", error);
                }
            }
        }
    }
}
