fn powmod(a: i32, b: i32, p: i32) -> i32 {
    let mut result = 1;
    let mut base = a % p;
    let mut exp = b;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % p;
        }
        base = (base * base) % p;
        exp /= 2;
    }

    result
}

fn main() {
    let p = 23;
    let alfa = 5;
    let a = 4;
    let b = 3;
    
    let A = powmod(alfa, a, p);
    let B = powmod(alfa, b, p);

    let KA = powmod(B, a, p);
    let KB = powmod(A, b, p);

    if KA == KB {
        println!("K = {}", KA);
    }
    else {
        println!("wrong calculation...");
    }
}
