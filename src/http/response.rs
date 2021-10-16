use super::StatusCode;
use std::io::{Result as IoResult, Write};

#[derive(Debug)]
pub struct Response {
    m_status_code: StatusCode,
    m_body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response {
            m_status_code: status_code,
            m_body: body,
        }
    }
    
    /* impl Write is how you implement static dispatching in Rust.
     * The compiler will look at the type 'Write' at compile time and statically
     * deduce that the TcpStream is correct type which needs to 'Write' trait.
     *
     * To switch this to dynamic dispatching, you would use 'dyn Write'
     * */
    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = match &self.m_body {
            Some(b) => b,
            None => "",
        };

        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.m_status_code,
            self.m_status_code.reason_phrase(),
            body
        )
    }
}
