pub mod server;
mod command;
mod handler;
mod request;

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum FtpError {
    IoError(std::io::Error),
    RequestError(String)
}

impl fmt::Display for FtpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "IO Error: {}", e),
            Self::RequestError(e) => write!(f, "Request Error: {}", e),
        }
    }
}

impl Error for FtpError {}

impl From<std::io::Error> for FtpError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

