pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    // unimplemented!(
    //     "Sum the multiples of all of {:?} which are less than {}",
    //     factors,
    //     limit
    // )
    let mut sum: u32 = 0;
    for i in 1..limit {
        for j in factors {
            if *j == 0 {
                continue;
            }
            if i % j == 0 {
                sum += i;
                break;
            }
        }
    }
    sum
}
