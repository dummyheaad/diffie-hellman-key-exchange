use curv::{BigInt, arithmetic::Zero};
use crate::utils::powmod;

pub struct DiffieHelman {
    p: BigInt,
    g: BigInt,
    private_key: BigInt,
    public_key: BigInt,
    shared_key: BigInt
}

impl DiffieHelman {
    pub fn set(&mut self, p: &BigInt, g: &BigInt, private_key: &BigInt) {
        self.p = p.clone();
        self.g = g.clone();
        self.private_key = private_key.clone();
    }

    pub fn set_public_key(&mut self) {
        self.public_key = powmod(&self.g, &self.private_key, &self.p);
    }

    pub fn set_shared_key(&mut self, key: &BigInt) {
        self.shared_key = powmod(&self.public_key, &key, &self.p);
    }

    pub fn get_shared_key(&self) -> BigInt {
        self.shared_key.clone()
    }

    pub fn get_private_key(&self) -> BigInt {
        self.private_key.clone()
    }
}

impl Default for DiffieHelman {
    fn default() -> Self {
        DiffieHelman {
            p: BigInt::zero(),
            g: BigInt::zero(),
            private_key: BigInt::zero(),
            public_key: BigInt::zero(),
            shared_key: BigInt::zero(),
        }
    }
}