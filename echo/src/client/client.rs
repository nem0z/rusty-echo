use std::net::TcpStream;
use std::io::{Error, Write, Read, BufReader, BufWriter};
use std::sync::{Arc,RwLock};

pub struct Client {
    stream: Arc<RwLock<TcpStream>>,
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>,
}

impl Client {
    pub fn new(uri: &str) -> Result<Self, Error> {
        let stream = TcpStream::connect(uri)?;
        let reader = BufReader::new(stream.try_clone()?);
        let writer = BufWriter::new(stream.try_clone()?);
        let stream_rw = Arc::new(RwLock::new(stream));

        return Ok(Client{ stream: stream_rw, reader: reader, writer: writer });
    }
    
    pub fn send_message(&mut self, msg: &str) -> Result<(), Error> {
        self.writer.write_all(msg.as_bytes())?;
        return Ok(());
    }

    pub fn read_message(&mut self) -> Result<String, Error> {
        let mut buffer = Vec::new();
        self.reader.read_to_end(&mut buffer)?;
        return Ok(String::from_utf8_lossy(&buffer).to_string());
    }
}