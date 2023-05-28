use curv::{BigInt, arithmetic::Zero};

mod prime;
mod utils;
mod affine;
mod diffie_helman;
mod crypto_utils;

fn main() {
    // IMPROVISASI: minta input bit_size, private_key_alice, private_key_bob, dan pesan / ciphertext

    let mut dh_alice = diffie_helman::DiffieHelman::default();
    let mut dh_bob = diffie_helman::DiffieHelman::default();
    let bit_size = 256;
    let p = prime::generate_random_prime(bit_size);
    // let p = BigInt::from(167);
    if prime::is_safe_prime(&p) {

        // ALGORITMA INI PERLU OPTIMASI
        let g = utils::primitive_root_modulo(&p).unwrap();

        let a = BigInt::from(25);
        dh_alice.set(&p, &g, &a);
        dh_alice.set_public_key();

        let b = BigInt::from(13);
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
        println!("=====ENKRIPSI/DEKRIPSI DIFFIE-HELLMAN VIA AFFINE CIPHER=====");
        println!("p = {}", p);
        println!("g = {}", g);
        println!("Private Key Alice = {}", dh_alice.get_private_key());
        println!("Private Key Bob = {}", dh_bob.get_private_key());
        println!("Shared Key = {}\n", shared_key);

        let mut affn = affine::AffineCipher::default();
        affn.set(&shared_key, &BigInt::zero(), &p);

        // let message = "Selamat ulang tahun yang ke-21, Aysuka Ansari !!!";
        let message = "abc";
        let message_bit_length = &message.as_bytes().len() * 8;
        println!("panjang bit pesan = {} bit", message_bit_length);
        println!("panjang bit p = {} bit", bit_size);
        if message_bit_length > bit_size.try_into().unwrap() {
            panic!("panjang bit pesan harus <= panjang bit p");
        }
        else {
            println!("Enkripsi bisa dilakukan!!!\n\n");
            
            println!("Message: {}", message);
    
            // ## VERSI 1
            // --> Enkripsi dilakukan ke keseluruhan pesan
            // Syarat: Perlu bilangan safe prime besar dengan jumlah bit >= bit pesan
            // ENKRIPSI
            let ciphertext = crypto_utils::encrypt_all_to_b64(&message, &affn);
            println!("Ciphertext: {}", ciphertext);
            
            // DEKRIPSI
            let plaintext = crypto_utils::decrypt_all_from_b64(&ciphertext, &mut affn);
            println!("Plaintext: {}", plaintext);
        }



        // ## VERSI 2
        // --> Enkripsi dilakukan per karakter pada pesan
        // Syarat: Perlu bilangan safe prime pada rentang [122 -255]
        // let ciphertext = crypto_utils::encrypt_to_b64(message, &affn);
        // println!("Encoded Message: {}", ciphertext);


        // let plaintext = crypto_utils::decrypt_from_b64(&encoded, &mut affn);
        // println!("Plaintext: {}", plaintext);
    }
    else {
        println!("p must be a safe prime");
    }
}
