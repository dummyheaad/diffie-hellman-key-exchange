use curv::{BigInt, arithmetic::Converter};
use base64;
use crate::utils::{affine_enc, affine_dec};
mod prime;
mod utils;


fn main() {
    // INGAT: BILANGAN PRIMA HARUS LEBIH BESAR DARI NILAI DATA TERBESAR
    let p = BigInt::from(167);
    if prime::is_safe_prime(&p) {
        let res_g = utils::primitive_root_modulo(&p);
        let g = match res_g {
            Ok(val) => val,
            Err(e) => panic!("Error: {}", e)
        };

        let private_a = BigInt::from(4);
        let private_b = BigInt::from(3);

        let public_a = utils::powmod(&g, &private_a, &p);
        let public_b = utils::powmod(&g, &private_b, &p);

        let key_a = utils::powmod(&public_b, &private_a, &p);
        let key_b = utils::powmod(&public_a, &private_b, &p);

        let shared_key = match key_a == key_b {
            true => key_a,
            false => panic!("Wrong calculation for shared key...")
        };

        // if key_a == key_b {
        //     println!("Shared key = {}", key_a);
        //     println!("Totient = {}", utils::euler_totient(&p));
        //     println!("p = {}", p);
        //     println!("g = {}", g);
        //     match utils::inv_mod(&BigInt::from(5), &BigInt::from(3)) {
        //         Ok(result) => println!("{}x = 1 mod {}, x = {}", BigInt::from(5), BigInt::from(3), result),
        //         Err(err) => println!("Error: {}", err)
        //     }
        // }
        // else {
        //     println!("Wrong calculation...");
        // }

        // Contoh Enkripsi dan Dekripsi sederhana via Affine Cipher
        println!("=====DEKRIPSI DIFFIE-HELLMAN VIA AFFINE CIPHER=====");
        println!("p = {}", p);
        println!("g = {}", g);
        println!("Private Key Uka = {}", private_a);
        println!("Private Key Akane = {}", private_b);
        println!("Shared Key = {}", shared_key);
        // let message = BigInt::from(27);
        // let ciphertext = utils::affine_enc(&shared_key, &message, &p);
        // println!("Hasil enkripsi: {}", ciphertext);

        // let plaintext = utils::affine_dec(&shared_key, &ciphertext, &p);
        // println!("Hasil dekripsi: {}", plaintext);

        // ENKRIPSI
        let message = "Selamat ulang tahun yang ke-21, Aysuka Ansari !!!";
        let mut message_vec: Vec<BigInt> = vec![];
        for ch in message.chars() {
            let message_ascii = BigInt::from(ch as u32);
            message_vec.push(message_ascii);
        }

        let mut ciphertext_vec: Vec<BigInt> = vec![];

        for m in &message_vec {
            let c = affine_enc(&shared_key, &m, &p);
            ciphertext_vec.push(c);
        }

        // println!("{:?}", message_vec);
        // println!("{:?}", ciphertext_vec);

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

        // println!("{}", ciphertext_hex);

        let encoded = base64::encode(ciphertext_hex);
        // println!("{}", encoded);
        let decoded_bytes = base64::decode(encoded.clone()).unwrap();
        let decoded_string = String::from_utf8(decoded_bytes).unwrap();
        // println!("{}", decoded_string);

        // DEKRIPSI
        let ciphertext = decoded_string;

        println!("\nSebelum Dekripsi: ");
        println!("Dari Akane: {}", encoded.clone());

        let mut decoded_ciphertext: Vec<BigInt> = Vec::new();

        for i in (0..ciphertext.len()).step_by(2) {
            if let Ok(hex_str) = &ciphertext[i..i+2].parse::<String>() {
                if let Ok(num) = BigInt::from_str_radix(hex_str, 16) {
                    decoded_ciphertext.push(num);
                }
            }
        }

        // println!("{:?}", decoded_ciphertext);

        let mut plaintext_vec: Vec<BigInt> = vec![];

        for dc in &decoded_ciphertext {
            let plain = affine_dec(&shared_key, &dc, &p);
            plaintext_vec.push(plain);
        }

        // println!("{:?}", plaintext_vec);

        let plaintext: String = plaintext_vec
            .iter()
            .map(|p| {
                let string = p.to_str_radix(10);
                let value = u32::from_str_radix(&string, 10).unwrap();
                char::from_u32(value).unwrap()
            })
            .collect();

        println!("\nHasil Dekripsi");
        println!("Dari Akane: {}", plaintext);
    }
    else {
        println!("p must be a safe prime");
    }
}
