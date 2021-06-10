pub fn build_proverb(list: &[&str]) -> String {
    // unimplemented!("build a proverb from this list of items: {:?}", list)
    let mut out = String::new();
    if list.len() > 0 {
        for i in 1..list.len() {
            out += &format!("For want of a {} the {} was lost.\n", list[i - 1], list[i]);
        }
        out += &format!("And all for the want of a {}.", list[0]);
    }
    out
}
