use std::collections::HashSet;
use unicode_segmentation::UnicodeSegmentation;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // unimplemented!(
    //     "For the '{}' word find anagrams among the following words: {:?}",
    //     word,
    //     possible_anagrams
    // );
    let mut anagrams = HashSet::new();
    let l = word.len();
    let wlwr = word.to_lowercase();
    let mut wsrt = wlwr.graphemes(true).collect::<Vec<&str>>();
    wsrt.sort();
    for p in possible_anagrams {
        let plwr = p.to_lowercase();
        if plwr != wlwr && plwr.len() == l {
            let mut psrt = plwr.graphemes(true).collect::<Vec<&str>>();
            psrt.sort();
            if psrt == wsrt {
                anagrams.insert(p.to_owned());
            }
        }
    }
    anagrams
}
