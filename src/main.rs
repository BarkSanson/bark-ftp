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

struct Command {
    // TODO
}

fn parse_command(buff: &[u8]) -> Result<(), Utf8Error> {
    let buff = match std::str::from_utf8(buff) {
        Ok(str) => str,
        Err(e) => return Err(e),
    };

    let buff: Vec<_> = buff.split(" ").collect();

    let command = buff[0];
    let args = &buff[1..];
    let args = args.join(" ");

    println!("Command: {}", command);
    println!("Args: {}", args);

    Ok(())
}

fn main() -> std::io::Result<()> {
    let socket_addr = SocketAddr::new(IpAddr::V4(HOST), CONTROL_PORT);
    let listener = TcpListener::bind(socket_addr)?;

    for stream in listener.incoming() {
        match stream {
            Ok(mut socket) => {
                // TODO: don't just unwrap ffs
                let addr = socket.peer_addr().unwrap();
                let ip = addr.ip();
                let port = addr.port();

                println!("Socket connected from IP {} with port {}", ip, port);

                thread::spawn(move || {
                    let mut buff: [u8; 50] = [0; 50];
                    loop {

                        let _ = socket.read(&mut buff);

                        parse_command(&buff);
                    }
                });
            },
            Err(e) => { eprintln!("ERROR: could not accept connection: {}", e); }
        }
    }

    Ok(())
}
