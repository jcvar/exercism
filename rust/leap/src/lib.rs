pub fn is_leap_year(year: u64) -> bool {
    // unimplemented!("true if {} is a leap year", year)
    if year % 400 == 0 {
        true
    } else if year % 100 == 0 {
        false
    } else if year % 4 == 0 {
        true
    } else {
        false
    }
}
