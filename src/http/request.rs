use super::method::Method;
use super::parse_error::ParseError;
use std::convert::TryFrom;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;

/* Request is a generic lifetime object, lifetimes
 * are denoted with the single qoute '
 * liftimes solve the dangling pointer, or dangling reference
 * problem that C/C++ have. The name 'buf' is made up. It is not
 * a built in keyword.
 * */
pub struct Request<'buf> {
    m_path: &'buf str,
    m_query_string: Option<&'buf str>,
    m_method: Method,
}

impl<'buf> Request<'buf> {
    fn get_next_word(request: &str) -> Option<(&str, &str)> {
        for (i, c) in request.chars().enumerate() {
            if c == ' ' || c == '\r' {
                return Some((&request[..i], &request[i + 1..]));
            }
        }
        None
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1\r\n
    fn try_from(buf: &'buf [u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buf)?;

        let (method, request) = Self::get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = Self::get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = Self::get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        dbg!("Local Method: {}", method);
        dbg!("Local Path: {}", path);
        dbg!("Local Protocol: {}", protocol);

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string: Option<&str> = None;
        /* use an if let statement instead of a regular Match,
         * as we don't care about the match arm where nothing is found
         * */
        if let Some(i) = path.find('?') {
            // grab everything after the '?'
            query_string = Some(&path[i + 1..]);
            // grab everything before the '?'
            path = &path[..i];
        }
        
        Ok(Self {
            m_path: path,
            m_query_string: query_string,
            m_method: method,
        })
    }
}

impl<'buf> Display for Request<'buf> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "Path: {}\nQuery String: {:#?}\nMethod: {:#?}",
            self.m_path, self.m_query_string, self.m_method
        )
    }
}

impl<'buf> Debug for Request<'buf> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "Path: {}\nQuery String: {:#?}\nMethod: {:#?}",
            self.m_path, self.m_query_string, self.m_method
        )
    }
}
