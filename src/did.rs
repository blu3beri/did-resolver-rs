use crate::error::{Error, Result};
use alloc::collections::BTreeMap;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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

#[derive(Serialize, Deserialize, Debug)]
pub struct DidDocument {
    #[serde(rename = "@context")]
    context: Vec<String>,
    id: String,
    also_known_as: Vec<String>,
    controller: Vec<String>,
    verification_method: Vec<VerificationMethod>,
    service: Vec<DidDocumentService>,
    authentication: Vec<VerificationMethod>,
    assert_method: Vec<VerificationMethod>,
    key_agreement: Vec<VerificationMethod>,
    capability_invocation: Vec<VerificationMethod>,
    capability_delegation: Vec<VerificationMethod>,
}

#[derive(Serialize, Deserialize, Debug)]
struct VerificationMethod {
    id: String,
    #[serde(rename = "type")]
    type_field: String,
    controller: String,
    public_key_base58: Option<String>,
    public_key_base64: Option<String>,
    public_key_jwk: Option<Value>,
    public_key_hex: Option<String>,
    public_key_multibase: Option<String>,
    public_key_pem: Option<String>,
    blockchain_account_id: Option<String>,
    ethereum_address: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct DidDocumentService {
    service_endpoint: String,
    id: String,
    #[serde(rename = "type")]
    type_field: String,
}

// export function parse(didUrl: string): ParsedDID | null {
//   if (didUrl === '' || !didUrl) return null
//   const sections = didUrl.match(DID_MATCHER)
//   if (sections) {
//     const parts: ParsedDID = {
//       did: `did:${sections[1]}:${sections[2]}`,
//       method: sections[1],
//       id: sections[2],
//       didUrl,
//     }
//     if (sections[4]) {
//       const params = sections[4].slice(1).split(';')
//       parts.params = {}
//       for (const p of params) {
//         const kv = p.split('=')
//         parts.params[kv[0]] = kv[1]
//       }
//     }
//     if (sections[6]) parts.path = sections[6]
//     if (sections[7]) parts.query = sections[7].slice(1)
//     if (sections[8]) parts.fragment = sections[8].slice(1)
//     return parts
//   }
//   return null
// }

//

pub fn parse_did(did: impl AsRef<str>) -> Result<Did> {
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
