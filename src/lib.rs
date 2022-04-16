extern crate alloc;

mod did;
mod error;
mod test;

use did::{parse_did, Did, DidDocument};
use error::{Error, Result};

#[derive(Debug, Eq, PartialEq)]
pub enum NumAlgo {
    InceptionKeyWithoutDoc,
    GenesisDoc,
    MultipleInceptionKeyWithoutDoc,
}

impl NumAlgo {
    fn from_u32(value: u32) -> Self {
        match value {
            0 => NumAlgo::InceptionKeyWithoutDoc,
            1 => NumAlgo::GenesisDoc,
            2 => NumAlgo::MultipleInceptionKeyWithoutDoc,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

#[derive(Debug)]
pub struct DidPeer {
    did: Did,
    did_document: Option<DidDocument>,
}

impl DidPeer {
    fn new(did: impl AsRef<str>, did_document: Option<DidDocument>) -> Result<Self> {
        let parsed_did = parse_did(did)?;

        Ok(Self {
            did: parsed_did,
            did_document,
        })
    }

    fn from_key(key: impl AsRef<str>) -> Result<Self> {
        DidPeer::new(format!("did:peer:0{}", key.as_ref()), None)
    }

    fn from_did(did: impl AsRef<str>) -> Result<Self> {
        DidPeer::new(did, None)
    }

    fn from_did_document(did_document: DidDocument) -> Result<Self> {
        todo!()
    }

    fn get_numalgo(did: impl AsRef<str>) -> Result<NumAlgo> {
        let did = did.as_ref();
        let mut parted = did.split(":");
        let numalgo = parted
            .nth(2)
            .ok_or(Error::UnableToGetNumAlgo)?
            .chars()
            .nth(0)
            .ok_or(Error::UnableToGetNumAlgo)?
            .to_digit(10)
            .ok_or(Error::UnableToGetNumAlgo)?;

        return Ok(NumAlgo::from_u32(numalgo));
    }
}
