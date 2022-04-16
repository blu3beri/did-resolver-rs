use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    UnableToParseDid,
    UnableToGetNumAlgo,
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UnableToParseDid => write!(f, "Unable to parse did"),
            UnableToParseDid => write!(f, "Unable to get num algo"),
        }
    }
}
