use std::{
    convert::TryFrom,
    error::Error,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    str,
    str::Utf8Error,
};
//Super Key import used to reference fn, struct in the directory level
use super::{method::MethodError, Method, QueryString, QueryStringValue};

#[derive(Debug)]
pub struct Request<'buff> {
    path: &'buff str,
    query_string: Option<QueryString<'buff>>,
    method: Method,
}

//Create getter so that we can be able to access values in request
impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn query_str(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }
}

// This tyform implementation has lifetime called buf because it is pointing to buffer slice in servers.rs file
// lifetime refer to as pointer that is depending onto some buffer which mean once that buffer is not currently available
// also that pointer can be deleted too this is automatic process.
//lifetime doesn't change the you call your method also you can consider it as metadata we give to compiler,
// It also help to gaurant the memory safety by pointer some reference that are pointing to some memory which me mean that they have to has same lifetime.
impl<'buff> TryFrom<&'buff [u8]> for Request<'buff> {
    type Error = ParseError;

    fn try_from(buf: &'buff [u8]) -> Result<Request<'buff>, Self::Error> {
        let request = str::from_utf8(buf)?;

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_str = None;
        // // First way of getting query string
        // match path.find('?') {
        //     Some(i) => {
        //         query_str = Some(&path[i + 1..]);
        //         path = &path[..i];
        //     }
        //     None => {}
        // }

        // // Second way of getting query string
        // let q = path.find('?');
        // if q.is_some() {
        //     let i = q.unwrap();
        //     query_str = Some(&path[i + 1..]);
        //     path = &path[..i];
        // }

        //Third way of getting query string
        if let Some(i) = path.find('?') {
            query_str = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        Ok(Self {
            path: path,
            query_string: query_str,
            method,
        })

        // unimplemented!()
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }

    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}
