use std::io::Error;
use std::net::{SocketAddr, TcpListener, TcpStream};
use crate::handler::Handler;

pub trait FtpServer {
    fn run(&mut self);
}

pub struct Server {
    control_socket: TcpListener,
    connections: Vec<TcpStream>
}

impl Server {
    pub fn new(control_addr: SocketAddr) -> Result<Self, Error> {
        let control_socket = TcpListener::bind(control_addr)?;

        Ok(Self {
            control_socket,
            connections: Vec::new()
        })
    }
}

impl FtpServer for Server {
    fn run(&mut self) {
        for stream in self.control_socket.incoming() {
            match stream {
                Ok(socket) => {
                    Handler::new(socket.try_clone().unwrap());
                    self.connections.push(socket);
                },
                Err(e) => eprintln!("Could not accept connection {}", e)
            }
        }
    }
}
