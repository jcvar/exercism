pub fn is_armstrong_number(num: u32) -> bool {
    // unimplemented!("true if {} is an armstrong number", num)
    let mut n = num;
    let mut p = 0;
    let mut arm = Vec::new();
    while n > 0 {
        arm.push(n%10);
        n/=10;
        p+=1;
    }
    num == arm.iter().map(|a| a.pow(p)).sum()
}
