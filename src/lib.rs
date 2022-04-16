extern crate alloc;

mod did;
mod error;

use did::{parse_did, Did, DidDocument};
use error::Result;

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
}

#[cfg(test)]
mod tests {
    use crate::DidPeer;

    #[test]
    fn debug() {
        let x = DidPeer::new(
            "did:peer:1zQmZdT2jawCX5T1RKUB7ro83gQuiKbuHwuHi8G1NypB8BTr",
            None,
        );
        println!("{:?}", x);
    }
}
