use super::method::Method;
use super::parse_error::ParseError;
use super::QueryString;
use std::convert::TryFrom;
use std::fmt::Debug;
use std::str;

/* Request is a generic lifetime object, lifetimes
 * are denoted with the single qoute '
 * liftimes solve the dangling pointer, or dangling reference
 * problem that C/C++ have. The name 'buf' is made up. It is not
 * a built in keyword.
 * */
#[derive(Debug)]
pub struct Request<'buf> {
    m_path: &'buf str,
    m_query_string: Option<QueryString<'buf>>,
    m_method: Method,
}

impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        &self.m_path
    }

    pub fn query_string(&self) -> Option<&QueryString<'buf>> {
        self.m_query_string.as_ref()
    }

    pub fn method(&self) -> &Method {
        &self.m_method
    }

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

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;
        /* use an if let statement instead of a regular Match,
         * as we don't care about the match arm where nothing is found
         * */
        if let Some(i) = path.find('?') {
            // grab everything after the '?'
            query_string = Some(QueryString::from(&path[i + 1..]));
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
