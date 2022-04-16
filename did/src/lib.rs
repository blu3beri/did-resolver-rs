extern crate alloc;

use crate::error::{Error, Result};
use alloc::collections::BTreeMap;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
mod error;

#[derive(Default, Debug)]
pub struct Did {
    pub did: String,
    pub did_url: String,
    pub method: String,
    pub id: String,
    pub path: Option<String>,
    pub fragment: Option<String>,
    pub query: Option<String>,
    pub params: BTreeMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DidDocument {
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    pub id: String,
    pub also_known_as: Vec<String>,
    pub controller: Vec<String>,
    pub verification_method: Vec<VerificationMethod>,
    pub service: Vec<DidDocumentService>,
    pub authentication: Vec<VerificationMethod>,
    pub assert_method: Vec<VerificationMethod>,
    pub key_agreement: Vec<VerificationMethod>,
    pub capability_invocation: Vec<VerificationMethod>,
    pub capability_delegation: Vec<VerificationMethod>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VerificationMethod {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub controller: String,
    pub public_key_base58: Option<String>,
    pub public_key_base64: Option<String>,
    pub public_key_jwk: Option<Value>,
    pub public_key_hex: Option<String>,
    pub public_key_multibase: Option<String>,
    pub public_key_pem: Option<String>,
    pub blockchain_account_id: Option<String>,
    pub ethereum_address: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DidDocumentService {
    pub service_endpoint: String,
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
}
impl Did {
    pub fn from_did(did: impl AsRef<str>) -> Result<Self> {
        let did = did.as_ref();
        if did.is_empty() {
            return Err(Error::UnableToParseDid.into());
        };

        let pct_encoded = "(?:%[0-9a-fA-F;]{2})";
        let id_char = format!("(?:[a-zA-Z0-9._-]|{})", pct_encoded);
        let method = "([a-z0-9]+)";
        let method_id = format!("((?:{}*:)*({}+))", id_char, id_char);
        let param_char = "[a-zA-Z0-9_.:%-]";
        let param = format!(";{}+={}*", param_char, param_char);
        let params = format!("(({})*)", param);
        let path = "(/[^#?]*)?";
        let query = "([?][^#]*)?";
        let fragment = "(#.*)?";
        let did_matcher = Regex::new(&format!(
            "^did:{}:{}{}{}{}{}$",
            method, method_id, params, path, query, fragment
        ))
        .unwrap();

        // TODO: path, fragment,query, params
        let captured = did_matcher.captures(did).ok_or(Error::UnableToParseDid)?;
        let did_url = captured.get(0).ok_or(Error::UnableToParseDid)?;
        let did_method = captured.get(1).ok_or(Error::UnableToParseDid)?;
        let id = captured.get(2).ok_or(Error::UnableToParseDid)?;

        let did = Did {
            did: format!("did:{}:{}", did_method.as_str(), id.as_str()),
            id: id.as_str().to_owned(),
            method: did_method.as_str().to_owned(),
            did_url: did_url.as_str().to_owned(),
            ..Default::default()
        };

        Ok(did)
    }
}
