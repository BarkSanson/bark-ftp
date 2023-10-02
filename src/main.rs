use std::net::{TcpListener, TcpStream, Ipv4Addr, SocketAddr, IpAddr, ToSocketAddrs};
use std::io::Write;
use std::thread;

const HOST: Ipv4Addr = Ipv4Addr::LOCALHOST;
const CONTROL_PORT: u16 = 5000;
const DATA_PORT: u16 = 5001;





struct FTPServer {
    control_socket: TcpListener,
    client_streams: Vec<TcpStream>
}

impl FTPServer {
    fn new<A: ToSocketAddrs>(addr: A) -> Result<Self, std::io::Error> {
        let listener = TcpListener::bind(addr)?;
        let server = Self { control_socket: listener, client_streams: vec![] };

        Ok(server)
    }

    fn run(&mut self) {
        for stream in self.control_socket.incoming() {
            match stream {
                Ok(socket) => {
                    self.client_streams.push(socket);

                    thread::spawn();
                },
                Err(e) => { eprintln!("ERROR: could not accept connection: {}", e); }
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) -> Result<(), std::io::Error> {
    stream.write_all(b"Welcome!")?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    let socket_addr = SocketAddr::new(IpAddr::V4(HOST), CONTROL_PORT);
    let listener = TcpListener::bind(socket_addr)?;

    for stream in listener.incoming() {
        handle_client(stream?).unwrap();
    }

    Ok(())
}
