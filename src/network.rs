use crate::*;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

pub struct Connection {
    pub stream: TcpStream,
}

impl Connection {
    pub fn new_server(addr: &str) -> Self {
        let (stream, _) = TcpListener::bind(addr)
            .unwrap()
            .accept()
            .expect("Could not bind to address");

        stream
            .set_nonblocking(true)
            .expect("Could not set non-blocking");

        Self { stream }
    }

    pub fn new_client(addr: &str) -> Self {
        let mut stream;

        loop {
            stream = match TcpStream::connect(addr) {
                Ok(stream) => Some(stream),
                Err(_) => None,
            };
            if stream.is_some() {
                break;
            }

            std::thread::sleep(Duration::from_secs(1));
        }

        let stream = stream.unwrap();

        stream
            .set_nonblocking(true)
            .expect("Could not set non-blocking");

        Self { stream }
    }

    fn _receive(&mut self) -> Vec<u8> {
        let mut data = [0u8; 1024];
        let res = self.stream.read(&mut data);

        match res {
            Ok(size) => data[..size].to_vec(),
            Err(_) => Vec::new(),
        }
    }

    fn _send(&mut self, data: Vec<u8>) {
        self.stream
            .write_all(&data)
            .expect("Could not write to stream");
    }

    pub fn send<T>(&mut self, s: T)
    where
        T: TryInto<Vec<u8>>,
    {
        let data: Vec<u8> = match s.try_into() {
            Ok(data) => data,
            Err(_) => {
                return;
            }
        };

        self._send(data);
    }

    pub fn receive<T>(&mut self) -> Option<T>
    where
        for<'a> T: TryFrom<&'a [u8]>,
    {
        let mut data = self._receive();

        if data.len() != 0 {
            println!("Received: {:?}", data);
        }

        let mut d = Vec::new();

        if data.len() == 1 {
            loop {
                std::thread::sleep(Duration::from_millis(10));
                d = self._receive();

                if d.len() != 0 {
                    println!("Received: {:?}", d);
                    break;
                }
            }
        }

        data.extend(d);

        let result = T::try_from(&data as &[u8]);

        match result {
            Ok(data) => Some(data),
            Err(_) => None,
        }
    }

    pub fn receive_skibidi<T>(&mut self) -> T
    where
        for<'a> T: TryFrom<&'a [u8]>,
    {
        let mut res: Option<T>;
        loop {
            res = self.receive();
            if res.is_some() {
                break;
            }
        }

        res.unwrap()
    }
}
