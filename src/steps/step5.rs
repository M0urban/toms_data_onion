use super::super::base85;
use openssl::{
    aes::{unwrap_key, AesKey},
    symm::{decrypt, Cipher},
};

pub fn run_step(payload: &[u8]) -> Vec<u8> {
    let encode = base85(payload);
    step5_extension(encode.as_slice())
}

fn step5_extension(payload: &[u8]) -> Vec<u8> {
    let kek = AesKey::new_decrypt(&payload[..32]).unwrap();
    let key_iv = Vec::from(&payload[32..40]);
    let real_key_enc = Vec::from(&payload[40..80]);
    let mut real_key = vec![0u8; 32];
    unwrap_key(
        &kek,
        Some(key_iv.try_into().unwrap()),
        &mut real_key,
        &real_key_enc,
    )
    .unwrap();

    let cipher = Cipher::aes_256_ctr();
    decrypt(cipher, &real_key, Some(&payload[80..96]), &payload[96..]).unwrap()
}
