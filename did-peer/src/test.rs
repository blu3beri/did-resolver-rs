#[cfg(test)]
mod tests {
    use did::DidDocument;

    use crate::{DidPeer, NumAlgo};

    #[test]
    fn did_peer_new_success() {
        let mock_did = "did:peer:1zQmZdT2jawCX5T1RKUB7ro83gQuiKbuHwuHi8G1NypB8BTr";
        let did = DidPeer::new(&mock_did, None);
        assert!(did.is_ok());
    }

    #[test]
    fn did_peer_new_fail() {
        let mock_did = "fake_did";
        let did = DidPeer::new(&mock_did, None);
        assert!(did.is_err());
    }

    #[test]
    fn did_peer_from_key() {
        let mock_key = "zQmZdT2jawCX5T1RKUB7ro83gQuiKbuHwuHi8G1NypB8BTr";
        let did = DidPeer::from_key(&mock_key).unwrap();
        assert!(did.did.did == "did:peer:0zQmZdT2jawCX5T1RKUB7ro83gQuiKbuHwuHi8G1NypB8BTr");
    }

    #[test]
    fn did_peer_from_did() {
        let mock_did = "did:peer:1zQmZdT2jawCX5T1RKUB7ro83gQuiKbuHwuHi8G1NypB8BTr";
        let did = DidPeer::from_did(&mock_did);
        assert!(did.is_ok());
    }

    #[test]
    fn did_peer_get_numalgo_success() {
        let mock_did = "did:peer:1zQmZdT2jawCX5T1RKUB7ro83gQuiKbuHwuHi8G1NypB8BTr";
        let numalgo = DidPeer::get_num_algo(mock_did).unwrap();
        assert!(numalgo == NumAlgo::GenesisDoc);
    }

    #[test]
    #[should_panic]
    fn did_peer_get_numalgo_fail() {
        let mock_did = "did:peer:8zQmZdT2jawCX5T1RKUB7ro83gQuiKbuHwuHi8G1NypB8BTr";
        let _ = DidPeer::get_num_algo(mock_did);
    }

    #[test]
    fn did_peer_from_did_document_fail() {
        let mock_did_document = DidDocument::default();
        let did = DidPeer::from_did_document(mock_did_document, None);
        assert!(did.is_err());
    }

    #[test]
    fn did_peer_from_did_document_num_algo_genesis_doc() {
        let mock_did_document = DidDocument::default();
        let did = DidPeer::from_did_document(mock_did_document, Some(NumAlgo::GenesisDoc));
        assert!(did.is_ok());
    }
}
