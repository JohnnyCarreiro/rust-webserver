use crate::http::{ParseReqError, Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

pub trait Handler {
    fn handle_request(&self, request: &Request) -> Response;
    fn handle_bad_request(&self, e: &ParseReqError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}
impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    // in this case we wanna give self ownership to the function, usually we use &self
    pub fn run(self, handler: impl Handler) {
        println!("Listening on http://{}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        // for conn in listener.incoming() {}

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_result) => {
                            // String::from_utf8_lossy returns a &str with utf8 chars,
                            // but  for non utf8 chars it returns a &str with question marks
                            println!("Request: {}", String::from_utf8_lossy(&buffer));

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(&request);
                                    handler.handle_request(&request)
                                }
                                Err(e) => handler.handle_bad_request(&e),
                            };

                            // this will send response, if err occurs it will be printed
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => {
                    println!("Failed to accept connection: {}", e);
                    continue;
                }
            }
        }
    }
}

// let res = listener.accept();
//
// if res.is_err() {
//     continue;
// }
//
// if let Ok((stream, _)) = res {
//     println!("Connection from: {}", stream.peer_addr().unwrap());
// }

// write!(stream, "{}", response).unwrap();
// let response = format!(
//     "{}\r\nContent-Length: {}\r\n\r\n{}",
//     "HTTP/1.1 200 OK",
//     &request.path().len(),
//     &request.path()
// );
// stream.write(response.as_bytes()).unwrap();
// stream.flush().unwrap();
