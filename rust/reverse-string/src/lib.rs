extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    // unimplemented!("Write a function to reverse {}", input);
    let mut output = String::with_capacity(input.len());
    let mut graphemes = input.graphemes(true);
    while let Some(g) = graphemes.next_back() {
        output += g;
    }
    output
}
