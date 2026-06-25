use aes_gcm::aead::rand_core::{RngCore,OsRng};
use ed25519_dalek::{SigningKey,VerifyingKey};
use hkdf::Hkdf;
use sha2::Sha256;

pub fn generate_keypair() -> (SigningKey, VerifyingKey){

    let mut secret = [0u8;32];
    OsRng.fill_bytes(&mut secret);
    let signing_key: SigningKey = SigningKey::from_bytes(&secret);
    let verification_key = signing_key.verifying_key();
    (signing_key,verification_key) 
}


pub fn derive_aes_key(master_secret: &[u8]) -> [u8;32]
{
    let hkdf: Hkdf<Sha256> = Hkdf::new(None,master_secret);
    let mut aes_key =[0u8;32];
    hkdf.expand(b"aes-key", &mut aes_key).unwrap();
    aes_key
}