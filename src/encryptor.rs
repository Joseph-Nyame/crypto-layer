use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce, Key 
};


// RECIPE: Encrypt some data //fun explanation
pub fn encrypt(data: &[u8], key: &[u8; 32]) -> anyhow::Result<(Vec<u8>, Vec<u8>)> {
    // Step 1: Take raw ingredients (key) 
    // Step 2: Prep them (convert to type-safe Key)
    let key_bytes = Key::<Aes256Gcm>::from_slice(key);
    
    // Step 3: Give prepped ingredients to chef (create cipher)
    let cipher = Aes256Gcm::new(key_bytes);
    
    // Step 4: Chef adds a secret spice (generates random nonce)
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    
    // Step 5: Chef cooks data! (encrypts)
    let ciphertext = cipher.encrypt(&nonce, data)
        .map_err(|e| anyhow::anyhow!("Encryption failed: {:?}", e))?;
    
    // Step 6: Serve the meal (return encrypted data + nonce)
    Ok((ciphertext, nonce.to_vec()))
}


pub fn decrypt(ciphertext: &[u8],key:&[u8;32],nonce: &[u8])->anyhow::Result<Vec<u8>>
{
    let key_bytes = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(key_bytes);
    let nonce = Nonce::from_slice(nonce);
    let plaintext = cipher.decrypt(nonce, ciphertext).map_err(|e| anyhow::anyhow!("Decryption failed:{:?}",e))?;
    Ok(plaintext)

}