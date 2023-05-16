use crate::http::{ParseError, Request, Response, StatusCode};
use std::{convert::TryFrom, convert::TryInto, io::Read, io::Write, net::TcpListener};

pub struct Server {
    addr: u16,
}

pub trait Handler {
    fn handler_request(&mut self, request: &Request) -> Response;
    fn handler_error(&mut self, e: &ParseError) -> Response {
        print!("Failed to error message {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

impl Server {
    pub fn new(addr: u16) -> Self {
        Self { addr }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on port {}", self.addr);

        let listener = TcpListener::bind(("127.0.0.1", self.addr)).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];

                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!(
                                "The recieved request is {}",
                                String::from_utf8_lossy(&buffer)
                            );

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handler_request(&request),
                                Err(e) => handler.handler_error(&e),
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response :{}", e);
                            }
                            // another way to do the same thing as above
                            // let res: &Result<Request, _> = &buffer[..].try_into();
                        }
                        Err(e) => {
                            println!("Error while reading request {}", e);
                        }
                    }

                    println!("Accepted{:?}", stream);
                }
                Err(e) => {
                    println!("Failed to establish a connection: {}", e);
                }
            }
        }
    }
}
