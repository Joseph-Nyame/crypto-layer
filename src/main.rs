mod key_manager;
mod encryptor;
mod signer;
mod envelope;
mod transport;

use key_manager::*;
use encryptor::*;
use signer::*;
use envelope::*;
use transport::*;


fn main() ->anyhow::Result<()>
{
    let msg ="my super secret msg";
    let (signing_key,verification_key) = generate_keypair();
    let aes_key =derive_aes_key("gingerbreadman".as_bytes());
    let (ciphertext,nonce) = encrypt(msg.as_bytes(),&aes_key)?;
    let signature = sign(&ciphertext, &signing_key);
    let envelope = Envelope {
        encrypted_payload: ciphertext,
        signature,
        nonce,
    };
    let json_string =envelope_to_json(&envelope)?;
    println!("JSON:{:?}",json_string);
    let envelope_reconstruct = json_to_envelope(&json_string)?;
    println!("Reconstructed envelope:{:?}",envelope_reconstruct);
   verify(&envelope_reconstruct.encrypted_payload,&envelope_reconstruct.signature,&verification_key)?;
   let plaintext =decrypt(&envelope.encrypted_payload,&aes_key,&envelope.nonce)?;
   println!("Decrypted data:{:?}",String::from_utf8(plaintext)?);
   Ok(())
}
