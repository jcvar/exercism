pub fn square_of_sum(n: u32) -> u32 {
    // unimplemented!("square of sum of 1...{}", n)
    (n * (n + 1) / 2).pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    // unimplemented!("sum of squares of 1...{}", n)
    n * (n + 1) * (2 * n + 1) / 6
}

pub fn difference(n: u32) -> u32 {
    // unimplemented!(
    //     "difference between square of sum of 1...{n} and sum of squares of 1...{n}",
    //     n = n,
    // )
    square_of_sum(n) - sum_of_squares(n)
}
