pub fn square(s: u32) -> u64 {
    // unimplemented!("grains of rice on square {}", s);
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64");
    }
    2u64.pow(s - 1)
}

pub fn total() -> u64 {
    // unimplemented!();
    (1..=64).fold(0, |mut s, i| -> u64 {
        s += square(i);
        s
    })
}
