use crate::*;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

pub struct Server {
    pub stream: TcpStream,
}

impl Server {
    pub fn new(addr: &str) -> Self {
        let listener = TcpListener::bind(addr).expect("Could not bind to address");

        let (stream, _addr) = listener.accept().expect("Could not accept connection");

        stream
            .set_nonblocking(true)
            .expect("Could not set non-blocking");

        Server { stream }
    }

    pub fn receive(&mut self) -> Vec<u8> {
        let mut data = [0u8; 1024];
        let res = self.stream.read(&mut data);

        match res {
            Ok(size) => data[..size].to_vec(),
            Err(err) => {
                println!("Error: {:?}", err);
                Vec::new()
            }
        }
    }

    pub fn send(&mut self, data: Vec<u8>) {
        self.stream
            .write_all(&data)
            .expect("Could not write to stream");
    }
}

pub struct Client {
    pub stream: TcpStream,
}

impl Client {
    pub fn new(addr: &str) -> Self {
        let stream = TcpStream::connect(addr).expect("Could not connect to server");

        stream
            .set_nonblocking(true)
            .expect("Could not set non-blocking");

        Client { stream }
    }

    pub fn receive(&mut self) -> Vec<u8> {
        let mut data = [0u8; 1024];
        let res = self.stream.read(&mut data);

        match res {
            Ok(size) => data[..size].to_vec(),
            Err(err) => {
                println!("Error: {:?}", err);
                Vec::new()
            }
        }
    }

    pub fn send(&mut self, data: Vec<u8>) {
        self.stream
            .write_all(&data)
            .expect("Could not write to stream");
    }
}
