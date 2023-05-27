use curv::{BigInt, arithmetic::Converter};
use crate::affine;
use base64::{Engine, engine::general_purpose};

pub fn encrypt_to_b64(message: &str, cryp: &affine::AffineCipher) -> String {
    let mut ciphertext_vec: Vec<BigInt> = vec![];
    for ch in message.chars() {
        let msg_ascii = BigInt::from(ch as u32);
        let c = cryp.encrypt(&msg_ascii);
        ciphertext_vec.push(c);
    }

    let ciphertext_hex: String = ciphertext_vec
        .iter()
        .map(|c| {
            let hex_c = c.to_str_radix(16);
            println!("{}", hex_c);
            if hex_c.len() == 1 {
                format!("0{}", hex_c)
            }
            else {
                hex_c
            }
        })
        .collect();

    let ciphertext_b64 = general_purpose::STANDARD.encode(&ciphertext_hex);
    ciphertext_b64
}

pub fn decrypt_from_b64(cipher: &String, cryp: &mut affine::AffineCipher) -> String {
    let decoded_bytes = general_purpose::STANDARD.decode(&cipher).unwrap();
    let decoded_string = String::from_utf8(decoded_bytes).unwrap();
    
    let mut decoded_ciphertext: Vec<BigInt> = Vec::new();
    for i in (0..decoded_string.len()).step_by(2) {
        match &decoded_string[i..i+2].parse::<String>() {
            Ok(hex_str) => {
                if let Ok(num) = BigInt::from_str_radix(hex_str, 16) {
                    decoded_ciphertext.push(num);
                }
            }
            Err(e) => {
                println!("Error {}", e);
            }
        }
    }
    
    let mut plaintext_vec: Vec<BigInt> = vec![];
    for dc in &decoded_ciphertext {
        let plain = cryp.decrypt(&dc);
        plaintext_vec.push(plain);
    }

    let plaintext: String = plaintext_vec
    .iter()
    .map(|p| {
        let string = p.to_str_radix(10);
        let value = u32::from_str_radix(&string, 10).unwrap();
        char::from_u32(value).unwrap()
    })
    .collect();

    plaintext
}