pub fn collatz(n: u64) -> Option<u64> {
    // unimplemented!(
    //     "return Some(x) where x is the number of steps required to reach 1 starting with {}",
    //     n,
    // )
    if n == 0 {
        return None;
    }

    let mut i = 0;
    let mut x = n;

    while x > 1 {
        if x % 2 == 0 {
            x /= 2;
        } else {
            x = 3 * x + 1;
        }
        i += 1;
    }
    Some(i)
}
