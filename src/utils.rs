use curv::{BigInt, arithmetic::Zero, arithmetic::One};
use crate::prime;

pub fn primitive_root_modulo(n: &BigInt) -> Result<BigInt, &'static str> {
    if *n < BigInt::from(5) {
        return Ok(BigInt::from(2));
    }

    // untuk sembarang nilai n
    // let totient: BigInt = euler_totient(&n);

    // khusus untuk n prima
    let totient = n.clone() - BigInt::one();

    // PERLU IMPROVISASI
    // let set_factors = prime::prime_factors(&totient);

    // HASIL IMPROVISASI
    // Alasan: n adalah safe prime sehingga selalu memiki 2 faktorisasi prima
    // yaitu 2 dan (n - 1)/2 [karena (n - 1)/2 juga merupakan prima maka tidak ada lagi faktorisasi prima yang lain]
    let set_factors = vec![BigInt::from(2), &totient/BigInt::from(2)];

    let mut g = BigInt::from(2);
    while g < *n {
        let mut is_primitive_root = false;
        for element in &set_factors {
            let exp = &totient / element;
            let result = powmod(&g, &exp, &n);
            if result == BigInt::one() {
                is_primitive_root = true;
                break;
            }
        }
        if is_primitive_root {
            return Ok(g);
        }
        g += 1;
    }

    Err("No primitive root found...")
}

pub fn powmod(a: &BigInt, b: &BigInt, p: &BigInt) -> BigInt {
    let mut result = BigInt::one();
    let mut base = a % p;
    let mut exp = b.clone();

    while &exp > &BigInt::zero() {
        if &exp % &BigInt::from(2) == BigInt::one() {
            result = (result * &base) % p.clone();
        }
        base = (&base * &base) % p.clone();
        exp /= BigInt::from(2);
    }

    result
}

pub fn euler_totient(n: &BigInt) -> BigInt {
    let mut result = n.clone();
    let prime_factors_set = prime::prime_set(&prime::prime_factors(&n));
    for element in &prime_factors_set {
        result -= &result / element;
    }
    result
}

pub fn inv_mod(a: &BigInt, n: &BigInt) -> Result<BigInt, &'static str> {
    let mut t = BigInt::zero();
    let mut r = n.clone();
    let mut new_t = BigInt::one();
    let mut new_r = a.clone();

    while &new_r != &BigInt::zero() {
        let quotient = &r / &new_r;
        let temp_t = new_t.clone();
        let temp_r = new_r.clone();
        new_t = &t - &quotient * &new_t;
        new_r = &r - &quotient * &new_r;
        t = temp_t;
        r = temp_r;
    }

    if &r > &BigInt::one() {
        return Err("a is not invertible");
    }
    if &t < &BigInt::zero() {
        t += n.clone();
    }

    Ok(t)
}