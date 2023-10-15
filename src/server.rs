use tokio::net::TcpListener;
use tokio::net::ToSocketAddrs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use bytes::BytesMut;

use crate::command::CommandType;
use crate::request::FtpRequest;


pub struct Server {
    control_socket: TcpListener,
}

impl Server {
    pub async fn new<T: ToSocketAddrs>(control_addr: T) -> Result<Self, std::io::Error> {
        let control_socket = TcpListener::bind(control_addr).await?;

        Ok(Self {
            control_socket,
        })
    }
}

impl Server {
    pub async fn run(&mut self) {
        loop {
            let (mut stream, _) = self.control_socket.accept().await.unwrap();

            tokio::spawn(async move {
                let mut buf = BytesMut::new();

                loop {
                    let n = match stream.read(&mut buf).await {
                        Ok(n) if n == 0 => return,
                        Ok(n) => n,
                        Err(e) => {
                            eprintln!("failed to read from socket; err = {:?}", e);
                            return;
                        }
                    };

                    let str_buf = String::from_utf8_lossy(&buf[..n]);
                    let request = FtpRequest::from_string(str_buf.to_string());


                    if let Err(err) = request {
                        // TODO: unwrap
                        let _ = stream.write(format!("Error parsing request: {}\n, err", err).as_bytes()).await.unwrap();
                        continue;
                    }

                    let request = request.unwrap();
                    match request.command {
                        CommandType::User=> {
                            // Fix this
                            let binding = request.arguments.unwrap();
                            let user = binding.first().unwrap();

                            // TODO: unwrap
                            let _ = stream
                                .write(format!("Authenticating user {}\n", user).as_bytes())
                                .await
                                .unwrap();
                        },
                        CommandType::Quit => {
                            let _ = stream.write(b"Quitting session...").await.unwrap();

                            // We will have to do this now,
                            // but socket closing should be more "gentle"
                            drop(stream);
                            break;
                        },
                        _ => unimplemented!("Command not implemented")
                    }

                }
            });

        }
    }
}
