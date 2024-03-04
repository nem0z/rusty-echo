use std::net::TcpStream;
use std::io::{Error, Write, Read};

pub struct Client {
    stream: TcpStream
}

impl Client {
    pub fn new(uri: &str) -> Result<Self, Error> {
        let stream = TcpStream::connect(uri)?;
        return Ok(Client{ stream: stream });
    }
    
    pub fn send_message(&mut self, msg: &str) -> Result<(), Error> {
        self.stream.write_all(msg.as_bytes())?;
        return Ok(());
    }

    pub fn read_message(&mut self) -> Result<String, Error> {
        let mut buffer = Vec::new();
        self.stream.read_to_end(&mut buffer)?;
        return Ok(String::from_utf8_lossy(&buffer).to_string());
    }
}