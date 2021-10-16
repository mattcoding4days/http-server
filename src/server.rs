use crate::http::{Request, Response, StatusCode, ParseError};
use std::convert::TryFrom;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use super::web_handler::WebSiteHandler;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, error: &ParseError) -> Response {
        println!("Failed to parse request: {}", error);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    m_addr: String,
    m_buff_size: usize,
    m_handler: WebSiteHandler,
}

impl Server {
    // associated functions are known as static functions in C++
    pub fn new(addr: String, handler: WebSiteHandler) -> Self {
        Server {
            m_addr: addr,
            m_buff_size: 1024,
            m_handler: handler,
        }
    }

    fn read_from_tcpstream(&mut self, stream: &mut TcpStream) {
        let mut buffer: Vec<u8> = vec![0; self.m_buff_size];
        match stream.read(&mut buffer) {
            Ok(_) => {
                println!(
                    "Received a request from: {}",
                    String::from_utf8_lossy(&buffer)
                );
                // try to convert
                let response = match Request::try_from(&buffer[..]) {
                    Ok(req) => self.m_handler.handle_request(&req),
                    Err(error) => self.m_handler.handle_bad_request(&error),
                };

                if let Err(error) = response.send(stream) {
                    println!("Request failed: {}", error);
                }
            }
            Err(error) => {
                println!("Failed to read from connection: {}", error);
            }
        }
    }

    pub fn run(&mut self) {
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
