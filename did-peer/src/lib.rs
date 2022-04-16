extern crate alloc;

mod error;
mod test;

use did::{Did, DidDocument};
use error::{Error, Result};
use multibase::Base;
use multihash::{Hasher, Sha2_256};

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
    pub did: Did,
    pub did_document: Option<DidDocument>,
}

impl DidPeer {
    pub fn new(did: impl AsRef<str>, did_document: Option<DidDocument>) -> Result<Self> {
        let parsed_did = Did::from_did(did)?;

        Ok(Self {
            did: parsed_did,
            did_document,
        })
    }

    pub fn from_key(key: impl AsRef<str>) -> Result<Self> {
        DidPeer::new(format!("did:peer:0{}", key.as_ref()), None)
    }

    pub fn from_did(did: impl AsRef<str>) -> Result<Self> {
        DidPeer::new(did, None)
    }

    pub fn from_did_document(
        did_document: DidDocument,
        mut num_algo: Option<NumAlgo>,
    ) -> Result<Self> {
        if num_algo.is_none() && did_document.id.starts_with("did:peer:") {
            num_algo = Self::get_num_algo(&did_document.id).ok();
        }

        if num_algo.is_none() {
            return Err(Error::UnableToGetNumalgo.into());
        }

        match num_algo.unwrap() {
            NumAlgo::GenesisDoc => {
                let s = serde_json::to_string(&did_document)?;

                let mut hasher = Sha2_256::default();
                hasher.update(s.as_bytes());
                let hash = hasher.finalize();
                let key = multibase::encode(Base::Base58Btc, hash);

                let did = format!("did:peer:1{}", key);

                DidPeer::new(did, Some(did_document))
            }
            NumAlgo::MultipleInceptionKeyWithoutDoc => {
                todo!()
            }
            _ => Err(Error::UnableToGetDidPeerFromDidDocument.into()),
        }
    }

    pub fn get_num_algo(did: impl AsRef<str>) -> Result<NumAlgo> {
        let did = did.as_ref();
        let mut parted = did.split(':');
        let numalgo = parted
            .nth(2)
            .ok_or(Error::UnableToGetNumalgo)?
            .chars()
            .next()
            .ok_or(Error::UnableToGetNumalgo)?
            .to_digit(10)
            .ok_or(Error::UnableToGetNumalgo)?;

        Ok(NumAlgo::from_u32(numalgo))
    }
}
