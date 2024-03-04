use std::net::{TcpListener, TcpStream};
use std::io::Error;

pub struct Server {
    listener: TcpListener,
    streams: Vec<TcpStream>
}

impl Server {
    pub fn new(uri: &str) -> Result<Self, Error> {
        let listener = TcpListener::bind(uri)?;
        return Ok(Self { listener: listener, streams: Vec::new() });
    }

    pub fn start(&mut self) {
        println!("Server listening on {}", self.listener.local_addr().unwrap());

        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    self.streams.push(stream);
                }
                Err(e) => {
                    eprintln!("Error accepting connection: {}", e);
                }
            }
        }
    }
}