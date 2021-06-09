pub fn nth(n: u32) -> u32 {
    // unimplemented!("What is the 0-indexed {}th prime number?", n)
    let mut cnt: u32 = 0;
    let mut num: u32 = 2;
    let mut cur: u32 = 3;

    while cnt < n {
        let mut div: u32 = 3;
        let mut prime = true;

        while div.pow(2) <= cur {
            if cur % div == 0 {
                prime = false;
                break;
            }
            div += 2;
        }

        if prime {
            num = cur;
            cnt += 1;
        }
        cur += 2;
    }
    num
}
