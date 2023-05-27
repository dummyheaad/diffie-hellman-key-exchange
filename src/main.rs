use curv::{BigInt, arithmetic::Converter, arithmetic::Zero};
use base64::{Engine, engine::general_purpose};

mod prime;
mod utils;
mod affine;
mod diffie_helman;

fn main() {

    let mut dh_alice = diffie_helman::DiffieHelman::default();
    let mut dh_bob = diffie_helman::DiffieHelman::default();

    // INGAT: BILANGAN PRIMA HARUS LEBIH BESAR DARI NILAI DATA TERBESAR
    // SETIAP PESAN YANG AKAN DIENKRIPSI HARUS < PRIMA
    let p = BigInt::from(167);
    if prime::is_safe_prime(&p) {
        let g = utils::primitive_root_modulo(&p).unwrap();

        let a = BigInt::from(4);
        dh_alice.set(&p, &g, &a);
        dh_alice.set_public_key();

        let b = BigInt::from(3);
        dh_bob.set(&p, &g, &b);
        dh_bob.set_public_key();

        let bob_private_key = dh_bob.get_private_key();
        dh_alice.set_shared_key(&bob_private_key);

        let alice_private_key = dh_alice.get_private_key();
        dh_bob.set_shared_key(&alice_private_key);

        let shared_key = match dh_alice.get_shared_key() == dh_bob.get_shared_key() {
            true => dh_alice.get_shared_key(),
            false => panic!("Wrong calculation")
        };

        // Contoh Enkripsi dan Dekripsi sederhana via Affine Cipher
        println!("=====DEKRIPSI DIFFIE-HELLMAN VIA AFFINE CIPHER=====");
        println!("p = {}", p);
        println!("g = {}", g);
        println!("Private Key Alice = {}", dh_alice.get_private_key());
        println!("Private Key Bob = {}", dh_bob.get_private_key());
        println!("Shared Key = {}", shared_key);

        let mut affn = affine::AffineCipher::default();
        affn.set(&shared_key, &BigInt::zero(), &p);

        // ENKRIPSI

        // let message = "Selamat ulang tahun yang ke-21, Aysuka Ansari !!!";
        let message = "I Love U So Much";

        let mut message_vec: Vec<BigInt> = vec![];
        for ch in message.chars() {
            let message_ascii = BigInt::from(ch as u32);
            message_vec.push(message_ascii);
        }

        let mut ciphertext_vec: Vec<BigInt> = vec![];
        for m in &message_vec {
            let c = affn.encrypt(&m);
            ciphertext_vec.push(c);
        }

        let ciphertext_hex: String = ciphertext_vec
        .iter()
        .map(|c| {
            let hex_string = c.to_str_radix(16);
            if hex_string.len() == 1 {
                format!("0{}", hex_string)
            } else {
                hex_string
            }
        })
        .collect();

        let encoded = general_purpose::STANDARD.encode(&ciphertext_hex);

        // DEKRIPSI
        
        let decoded_bytes = general_purpose::STANDARD.decode(&encoded).unwrap();
        let decoded_string = String::from_utf8(decoded_bytes).unwrap();
        let ciphertext = decoded_string;

        println!("\nSebelum Dekripsi: ");
        println!("Dari Alice: {}", encoded.clone());

        let mut decoded_ciphertext: Vec<BigInt> = Vec::new();
        for i in (0..ciphertext.len()).step_by(2) {
            if let Ok(hex_str) = &ciphertext[i..i+2].parse::<String>() {
                if let Ok(num) = BigInt::from_str_radix(hex_str, 16) {
                    decoded_ciphertext.push(num);
                }
            }
        }

        let mut plaintext_vec: Vec<BigInt> = vec![];
        for dc in &decoded_ciphertext {
            let plain = affn.decrypt(&dc);
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

        println!("\nHasil Dekripsi");
        println!("Dari Alice: {}", plaintext);
    }
    else {
        println!("p must be a safe prime");
    }
}
