mod command;

use std::io::Read;
use std::net::{TcpListener, Ipv4Addr, SocketAddr, IpAddr, TcpStream};
//use std::io::Write;
use std::{io, thread};
use std::str::Utf8Error;

const HOST: Ipv4Addr = Ipv4Addr::LOCALHOST;
const CONTROL_PORT: u16 = 5000;

// Temporal username to start implementing
// USER command
const USER: &str = "bark";

trait FtpServer {
    fn run(&self);
}

struct Server {
    control_socket: TcpListener
}

impl Server {
    fn new(control_addr: SocketAddr) -> Result<Self, io::Error> {
        let control_socket = TcpListener::bind(control_addr)?;

        Ok(Self {
            control_socket
        })
    }
}

impl FtpServer for Server {
    fn run(&self) {
        for stream in self.control_socket.incoming() {
            match stream {
                Ok(mut socket) => {
                    thread::spawn(move || {
                        let mut buff: [u8; 50] = [0; 50];
                        loop {

                            // TODO: don't just unwrap ffs
                            let bytes = socket.read(&mut buff).unwrap();

                            if bytes <= 0 {
                                break;
                            }

                            parse_command(&buff);

                            buff = [0; 50];
                        }
                    });
                },
                Err(e) => eprintln!("Could not accept connection {}", e)
            }
        }
    }
}

fn parse_command(buff: &[u8]) -> Result<(), Utf8Error> {
    let buff = match std::str::from_utf8(buff) {
        Ok(str) => str,
        Err(e) => return Err(e),
    };

    let buff: Vec<_> = buff.split(" ").collect();

    println!("{}", buff.join(" "));

    let command = buff[0];

    let args = &buff[1..];
    let args = args.join(" ");

    println!("Command: {}", command);
    println!("Args: {}", args);

    Ok(())
}

fn main() -> std::io::Result<()> {
    let socket_addr = SocketAddr::new(IpAddr::V4(HOST), CONTROL_PORT);

    let server = Server::new(socket_addr)?;

    server.run();

    Ok(())
}
