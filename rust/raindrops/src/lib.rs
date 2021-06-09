pub fn raindrops(n: u32) -> String {
    // unimplemented!("what sound does Raindrop #{} make?", n)
    let mut out = String::new();

    if n % 3 == 0 {
        out += "Pling";
    }
    if n % 5 == 0 {
        out += "Plang";
    }
    if n % 7 == 0 {
        out += "Plong";
    }
    if out.is_empty() {
        out = n.to_string();
    }
    out
}
