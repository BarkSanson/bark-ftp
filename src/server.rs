use std::io::{Error, Read, Write};
use std::net::{SocketAddr, TcpListener};
use std::thread;
use crate::command::CommandType;
use crate::request::{FtpRequest, self};

pub trait FtpServer {
    fn run(&mut self);
}

pub struct Server {
    control_socket: TcpListener,
}

impl Server {
    pub fn new(control_addr: SocketAddr) -> Result<Self, Error> {
        let control_socket = TcpListener::bind(control_addr)?;

        Ok(Self {
            control_socket,
        })
    }
}

impl FtpServer for Server {
    fn run(&mut self) {
        for stream in self.control_socket.incoming() {
            match stream {
                Ok(mut socket) => {
                    thread::spawn(move || {
                        loop {
                            let mut buff: [u8; 1024] = [0; 1024];
                            match socket.read(&mut buff) {
                                // Closed connection
                                Ok(0) => {
                                    // Terminate connection with client
                                    drop(socket);
                                    break;
                                },
                                Ok(_) => {},
                                Err(e) => panic!("{}", e),
                            };

                            let str_buff = String::from_utf8_lossy(&buff);
                            let request = FtpRequest::from_string(str_buff.to_string());

                            if let Err(err) = request {
                                // TODO: unwrap
                                socket.write_all(format!("{}\n", err).as_bytes()).unwrap();
                                continue;
                            }

                            let request = request.unwrap(); 

                            match request.command {
                                 CommandType::User=> {
                                    // Fix this
                                    let binding = request.arguments.unwrap();
                                    let user = binding.first().unwrap();

                                    let _ = socket
                                        .write_all(format!("Authenticating user {}\n", user)
                                        .as_bytes());
                                },
                                CommandType::Quit => {
                                    let _ = socket
                                        .write_all("Quitting session...\n".as_bytes());

                                    // We will have to do this now,
                                    // but socket closing should be more "gentle"
                                    drop(socket);
                                    break;
                                },
                                _ => unimplemented!("Command not implemented")
                            }

                            // Reset buff so it can read fresh new data
                            // from socket
                            buff = [0; 1024];
                        }
                    });
                },
                Err(e) => eprintln!("Could not accept connection {}", e)
            }
        }
    }
}
