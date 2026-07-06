use base64::{Engine, engine::general_purpose::STANDARD};
use crate::envelope::Envelope;


pub fn envelope_to_json(envelope:&Envelope)->anyhow::Result<String>
{
    let transport =TransportEnvelope {
        encrypted_payload:STANDARD.encode(&envelope.encrypted_payload),
        signature:STANDARD.encode(&envelope.signature),
        nonce:STANDARD.encode(&envelope.nonce),
    };
    Ok(serde_json::to_string(&transport)?)

}


pub fn json_to_envelope(json:&str)->anyhow::Result<Envelope>
{
    let transport:TransportEnvelope = serde_json::from_str(json)?;
    let envelope =Envelope{
        encrypted_payload:STANDARD.decode(&transport.encrypted_payload)?,
        signature:STANDARD.decode(&transport.signature)?,
        nonce:STANDARD.decode(&transport.nonce)?,
    };
    Ok(envelope)
}


#[derive(serde::Serialize,serde::Deserialize)]
pub struct TransportEnvelope{
    encrypted_payload:String,
    signature:String,
    nonce:String,
}