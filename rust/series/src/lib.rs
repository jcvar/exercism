pub fn series(digits: &str, len: usize) -> Vec<String> {
    // unimplemented!(
    //     "What are the series of length {} in string {:?}",
    //     len,
    //     digits
    // )
    let mut series = Vec::new();
    if len <= digits.len() {
        for i in 0..=(digits.len() - len) {
            if let Some(s) = digits.get(i..i + len) {
                series.push(s.to_string());
            }
        }
    }
    series
}
