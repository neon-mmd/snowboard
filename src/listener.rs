use std::{
    io::{Error, ErrorKind, Read, Result},
    net::{TcpListener, TcpStream},
};

use crate::{server::DEFAULT_BUFFER_SIZE, Request, Response};

/// Single threaded listener made for simpler servers.
#[derive(Debug)]
pub struct Listener {
    tcp_listener: TcpListener,
    pub buffer_size: usize,
}

impl Listener {
    pub fn new<'a>(addr: impl Into<&'a str>) -> Self {
        let addr = addr.into();
        Self {
            tcp_listener: TcpListener::bind(addr).unwrap(),
            buffer_size: DEFAULT_BUFFER_SIZE,
        }
    }

    pub fn set_buffer_size(&mut self, size: usize) {
        assert!(size > 0, "Buffer size must be greater than 0");

        self.buffer_size = size;
    }

    pub fn try_accept(&self) -> Result<(TcpStream, Request)> {
        let stream = self.tcp_listener.accept()?;

        let (mut stream, ip) = stream;

        let mut buffer = vec![0; self.buffer_size];
        let payload_size = stream.read(&mut buffer)?;

        if payload_size > self.buffer_size {
            Response::payload_too_large(None, None, None).send_to(&mut stream);
            return Err(Error::new(ErrorKind::InvalidInput, "Payload too large"));
        }

        let text = String::from_utf8_lossy(&buffer).replace('\0', "");

        Ok((stream, Request::new(text, ip)))
    }
}

impl Iterator for Listener {
    type Item = (TcpStream, Request);

    fn next(&mut self) -> Option<Self::Item> {
        match self.try_accept() {
            Ok(req) => Some(req),
            Err(_) => None,
        }
    }
}