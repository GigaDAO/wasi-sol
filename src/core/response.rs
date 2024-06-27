use serde::{Deserialize, Serialize};

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
