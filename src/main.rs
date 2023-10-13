use std::env;
use std::net::{Ipv4Addr, SocketAddr, IpAddr};
use std::process::exit;
use std::str::FromStr;
use bark_ftp::server::{FtpServer, Server};

fn parse_address(addr: &str) -> IpAddr {
    // TODO: unwrap
    let result = Ipv4Addr::from_str(addr).unwrap();
    IpAddr::V4(result)
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Incorrect usage. Correct usage: ");
        eprintln!("\tbark-ftp <host> <port>");
        exit(1);
    }

    let binding = args[1].to_lowercase();
    let host = binding.trim();
    let host = match host {
        "localhost" | "127.0.0.1" => IpAddr::V4(Ipv4Addr::LOCALHOST),
        _ => parse_address(host)
    };

    // TODO: check unwrap
    let port = args[2].parse::<u16>().unwrap();

    let socket_addr = SocketAddr::new(host, port);

    let mut server = Server::new(socket_addr)?;

    server.run();

    Ok(())
}
