use crate::http::Request;
use std::convert;
use std::io::Read;
use std::net::TcpListener;
use std::thread::Result;
pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Self { address }
    }

    pub fn run(self) {
        println!("Listening on {}", self.address);
        let listener = TcpListener::bind(&self.address).unwrap();

        // while true {}
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {}
                                Err(e) => println!("Failed to parse request: {}", e),
                            } // or (&buffer as &[u8])
                        }
                        Err(e) => println!("Failed to read connection: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish connection: {}", e),
            }
            // let accept = listener.accept();
            // if accept.is_err() {
            //     continue;
            // }
            //
            // let (stream, address) = accept.unwrap();
        }
    }
}
