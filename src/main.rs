use std::{collections::HashSet};

fn is_prime(n: &u32) -> bool {
    if *n <= 3 {
        return false;
    }

    if (*n % 2 == 0) || (*n % 3 == 0) {
        return false;
    }

    let mut i: u32 = 5;
    while i * i <= *n {
        if (*n % i == 0) || (*n % i + 2) == 0 {
            return false;
        }
        i += 6;
    }

    true
}

fn prime_set(v: &Vec<u32>) -> HashSet<u32> {
    let set = v.iter().cloned().collect();
    set
}

fn prime_factors(n: &u32) -> HashSet<u32> {
    let mut factors = Vec::new();
    let mut num = *n;
    let mut divisor = 2;

    while divisor * divisor <= num {
        if num % divisor == 0 {
            factors.push(divisor);
            num /= divisor;
        }
        else {
            divisor += 1;
        }
    }

    if num > 1 {
        factors.push(num);
    }

    let set_factors: HashSet<u32> = prime_set(&factors);

    set_factors
}

fn primitive_root_modulo(n: &u32) -> u32 {
    if *n < 5 {
        return 2;
    }

    // asumsi n adalah prima
    let torsion_n: u32 = *n - 1;
    let set_factors: HashSet<u32> = prime_factors(&torsion_n);

    for g in 2..*n {
        let mut is_primitive_root = false;
        for element in &set_factors {
            let exp: u32 = torsion_n / element;
            let result: u32 = powmod(&g, &exp, &n);
            if result == 1 {
                is_primitive_root = true;
                break;
            }
        }
        if is_primitive_root {
            return g;
        }
    }

    unreachable!();
}

fn powmod(a: &u32, b: &u32, p: &u32) -> u32 {
    let mut result = 1;
    let mut base = *a % *p;
    let mut exp = *b;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % p;
        }
        base = (base * base) % *p;
        exp /= 2;
    }

    result
}

fn main() {
    let p: u32 = 41;
    if is_prime(&p) {
        let g: u32 = primitive_root_modulo(&p);
        let private_a: u32 = 4;
        let private_b: u32 = 3;

        let public_a: u32 = powmod(&g, &private_a, &p);
        let public_b: u32 = powmod(&g, &private_b, &p);

        let key_a = powmod(&public_b, &private_a, &p);
        let key_b = powmod(&public_a, &private_b, &p);

        if key_a == key_b {
            println!("Shared key = {}", key_a);
            println!("p = {}", p);
            println!("g = {}", g);
        }
        else {
            println!("Wrong calculation...");
        }

        // Contoh Enkripsi dan Dekripsi sederhana via Affine Cipher
    }
    else {
        println!("p must be a prime");
    }
}
