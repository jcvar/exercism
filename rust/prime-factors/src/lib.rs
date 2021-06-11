pub fn factors(n: u64) -> Vec<u64> {
    // unimplemented!("This should calculate the prime factors of {}", n)
    let mut cur = n;
    let mut fctrs = Vec::new();
    while cur % 2 == 0 {
        cur >>= 1;
        fctrs.push(2);
    }
    let mut nxt = 3;
    while cur > 1 {
        while cur % nxt == 0 {
            cur /= nxt;
            fctrs.push(nxt);
        }
        nxt += 2; // next_prime(nxt);
    }
    fctrs
}

// fn next_prime(p: u64) -> u64 {
//     let mut n = p;
//     let mut is_n;
//     let mut d: u64;
//     loop {
//         is_n = true;
//         d = 3;
//         while d.pow(2) <= n {
//             if n % d == 0 {
//                 is_n = false;
//                 break;
//             }
//             d += 2;
//         }
//         if is_n {
//             return n;
//         }
//         n += 2;
//     }
// }
