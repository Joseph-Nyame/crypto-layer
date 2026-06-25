use ed25519_dalek::{Signature, Signer, Verifier};

pub fn sign(payload: &[u8],signing_key:&ed25519_dalek::SigningKey)->Vec<u8>
{
    let signature = signing_key.sign(payload);
    signature.to_vec()
}


pub fn verify(payload: &[u8],signature: &[u8],verification_key:&ed25519_dalek::VerifyingKey) ->anyhow::Result<()>
{
    let sig_bytes:[u8;64] = signature.try_into().map_err(|_| anyhow::anyhow!("Invalid signature length"))?;
    let signature_converted = Signature::from_bytes(&sig_bytes);
    verification_key.verify(payload, &signature_converted).map_err(|e| anyhow::anyhow!("Signature verification failed: {:?}", e))?;
    Ok(())
}