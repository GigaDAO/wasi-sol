use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct SignedMessage {
    #[serde(rename = "type")]
    message_type: String,
    data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Signature {
    #[serde(rename = "type")]
    signature_type: String,
    data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageObject {
    address: String,
    #[serde(rename = "SignedMessage")]
    signed_message: SignedMessage,
    signature: Signature,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsSignature {
    #[serde(rename = "type")]
    pub sig_type: String,
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsSignatureResponse {
    pub signature: JsSignature,
    #[serde(rename = "publicKey")]
    pub public_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignaturesObject {
    pub signatures: Vec<SignatureArray>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignatureArray {
    #[serde(flatten)]
    pub signature: serde_json::Map<String, Value>,
}

#[derive(Deserialize, Debug)]
pub struct JsSignatureObject {
    pub signature: String,
    #[serde(rename = "publicKey")]
    pub public_key: String,
}
