use std::io::Write;
use std::net::{TcpListener, Ipv4Addr, SocketAddr, IpAddr};
//use std::io::Write;
use std::thread;

const HOST: Ipv4Addr = Ipv4Addr::LOCALHOST;
const CONTROL_PORT: u16 = 5000;

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
                    socket.write_all(b"Hello, world!").unwrap();
                    loop {}
                });
            },
            Err(e) => { eprintln!("ERROR: could not accept connection: {}", e); }
        }
    }

    Ok(())
}
