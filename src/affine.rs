use curv::{BigInt, arithmetic::Zero};
use crate::utils::inv_mod;
pub struct AffineCipher {
    a: BigInt,
    inv_a: BigInt,
    b: BigInt,
    n: BigInt
}


impl AffineCipher {
    pub fn set(&mut self, a: &BigInt, b: &BigInt, n: &BigInt) {
        self.a = a.clone();
        self.b = b.clone();
        self.n = n.clone();
    }

    pub fn encrypt(&self, x: &BigInt) -> BigInt {
        (((self.a.clone() % self.n.clone()) * (x % self.n.clone())) + self.b.clone() % self.n.clone()) % self.n.clone()
    }

    pub fn decrypt(&mut self, y: &BigInt) -> BigInt {
        self.inv_a = inv_mod(&self.a, &self.n).unwrap();
        (self.inv_a.clone() % self.n.clone()) * (y - self.b.clone()) % self.n.clone()
    }
}

impl Default for AffineCipher {
    fn default() -> Self {
        AffineCipher { 
            a: BigInt::zero(),
            inv_a: BigInt::zero(),
            b: BigInt::zero(),
            n: BigInt::zero()
        }
    }
}