use std::io::Read;
use std::net::TcpStream;
use std::thread;

pub struct Handler {
    control_socket: TcpStream,
    thread: thread::JoinHandle<()>
}

impl Handler {
    pub fn new(control_socket: TcpStream) -> Self {
        Self {
            control_socket,
            thread
        }
    }

    pub fn handle(mut self) {
        let thread = thread::spawn(||{
            loop {
                let mut buff: [u8; 1024] = [0; 1024];

                if let Ok(bytes) = self.control_socket.read(&mut buff) {
                    // Closed connection
                    if bytes == 0 {
                        // Notify parent and stop thread
                        todo!()
                    } else {
                        todo!()
                    }
                } else {
                    todo!()
                }

            }
        });
    }
}