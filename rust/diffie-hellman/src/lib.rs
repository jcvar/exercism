use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    // unimplemented!("Pick a private key greater than 1 and less than {}", p)
    rand::thread_rng().gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    // unimplemented!(
    //     "Calculate public key using prime numbers {} and {}, and private key {}",
    //     p,
    //     g,
    //     a
    // )
    mod_pow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    // unimplemented!(
    //     "Calculate secret key using prime number {}, public key {}, and private key {}",
    //     p,
    //     b_pub,
    //     a
    // )
    mod_pow(b_pub, a, p)
}

pub fn mod_pow(b: u64, e: u64, m: u64) -> u64 {
    if m == 1 {
        0
    } else {
        let mut exp = e;
        let mut bse = b % m;
        let mut res = 1;
        while exp > 0 {
            if exp % 2 == 1 {
                res = (res * bse) % m;
            }
            exp /= 2;
            bse = bse.pow(2) % m;
        }
        res
    }
}
