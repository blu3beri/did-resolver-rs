use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    UnableToGetNumalgo,
    UnableToGetDidPeerFromDidDocument,
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnableToGetNumalgo => write!(f, "Unable to get num algo"),
            Error::UnableToGetDidPeerFromDidDocument => {
                write!(f, "Unable to get did-peer from did document")
            }
        }
    }
}
