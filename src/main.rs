use curv::{BigInt, arithmetic::Zero};

mod prime;
mod utils;
mod affine;
mod diffie_helman;
mod crypto_utils;

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
        // MASALAH: NILAI P GA BISA TERLALU GEDE.
        // IMPROVISASI: BIKIN AGAR NILAI P NYA BISA GEDE DAN BERHASIL DEKRIPSI PESAN
        println!("=====ENKRIPSI/DEKRIPSI DIFFIE-HELLMAN VIA AFFINE CIPHER=====");
        println!("p = {}", p);
        println!("g = {}", g);
        println!("Private Key Alice = {}", dh_alice.get_private_key());
        println!("Private Key Bob = {}", dh_bob.get_private_key());
        println!("Shared Key = {}", shared_key);

        let mut affn = affine::AffineCipher::default();
        affn.set(&shared_key, &BigInt::zero(), &p);

        // ENKRIPSI

        // let message = "Selamat ulang tahun yang ke-21, Aysuka Ansari !!!";
        let message = "I Love U So Much!!!";
        println!("Message: {}", message);
        let encoded = crypto_utils::encrypt_to_b64(message, &affn);
        println!("Encoded Message: {}", encoded);

        // DEKRIPSI
        let plaintext = crypto_utils::decrypt_from_b64(&encoded, &mut affn);
        println!("Plaintext: {}", plaintext);
    }
    else {
        println!("p must be a safe prime");
    }
}
