use std::collections::HashSet;
use num_bigint::BigInt;
use num_traits::ToPrimitive;

fn is_prime(n: &BigInt) -> bool {
    // Ganti ke miller rabin test
    if *n <= BigInt::from(3) {
        return false;
    }

    if (n % BigInt::from(2) == BigInt::from(0)) || (n % BigInt::from(3) == BigInt::from(0)) {
        return false;
    }

    let mut i: BigInt = BigInt::from(5);
    while &i * &i <= *n {
        if (n % &i == BigInt::from(0)) || (n % &i + 2) == BigInt::from(0) {
            return false;
        }
        i += 6;
    }

    true
}

fn prime_set(v: &Vec<BigInt>) -> HashSet<BigInt> {
    let set: HashSet<BigInt> = v.iter().cloned().collect();
    set
}

fn prime_factors(n: &BigInt) -> Vec<BigInt> {
    let mut factors = Vec::new();
    let mut num = n.clone();
    let mut divisor = BigInt::from(2);
    
    while &divisor * &divisor <= num {
        if &num % &divisor == BigInt::from(0) {
            factors.push(divisor.clone());
            num /= &divisor;
        }
        else {
            divisor += 1;
        }
    }

    if &num > &BigInt::from(1) {
        factors.push(num);
    }

    factors
}

fn primitive_root_modulo(n: &BigInt) -> BigInt {
    if *n < BigInt::from(5) {
        return BigInt::from(2);
    }

    let totient: BigInt = n.clone() - BigInt::from(1);
    let set_factors: HashSet<BigInt> = prime_set(&prime_factors(&totient));

    let mut g = BigInt::from(2);
    while g < *n {
        let mut is_primitive_root = false;
        for element in &set_factors {
            let exp = &totient / element;
            let result = powmod(&g, &exp, &n);
            if result == BigInt::from(1) {
                is_primitive_root = true;
                break;
            }
        }
        if is_primitive_root {
            return g;
        }
        g += 1;
    }

    unreachable!();
}

fn powmod(a: &BigInt, b: &BigInt, p: &BigInt) -> BigInt {
    let mut result = BigInt::from(1);
    let mut base = a % p;
    let mut exp = b.clone();

    while &exp > &BigInt::from(0) {
        if &exp % &BigInt::from(2) == BigInt::from(1) {
            result = (result * &base) % p.clone();
        }
        base = (&base * &base) % p.clone();
        exp /= BigInt::from(2);
    }

    result
}

fn euler_totient(n: &BigInt) -> BigInt {
    let mut result = n.clone();
    let prime_factors_set = prime_set(&prime_factors(&n));
    for element in &prime_factors_set {
        result -= &result / element;
    }
    result
}

fn main() {
    let p = BigInt::from(17);
    if is_prime(&p) {
        let g = primitive_root_modulo(&p);
        let private_a = BigInt::from(4);
        let private_b = BigInt::from(3);

        let public_a = powmod(&g, &private_a, &p);
        let public_b = powmod(&g, &private_b, &p);

        let key_a = powmod(&public_b, &private_a, &p);
        let key_b = powmod(&public_a, &private_b, &p);

        if key_a == key_b {
            println!("Shared key = {}", key_a);
            println!("Totient = {}", euler_totient(&p));
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
