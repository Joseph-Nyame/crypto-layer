
#[derive(Debug, serde::Serialize,serde::Deserialize)]
pub struct Envelope {
  pub encrypted_payload: Vec<u8>,
  pub signature: Vec<u8>,
  pub nonce: Vec<u8>,
}