use std::collections::HashSet;
use unicode_segmentation::UnicodeSegmentation;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut set: HashSet<&str> = HashSet::new();
    let word = word.to_lowercase();
    let mut word_graphemes = word.graphemes(true).collect::<Vec<&str>>();
    word_graphemes.sort_unstable();

    for anagram in possible_anagrams {
        let anagram_copy = anagram.to_lowercase();
        if word == anagram_copy {
            continue;
        }
        let mut anagram_graphemes = anagram_copy.graphemes(true).collect::<Vec<&str>>();
        anagram_graphemes.sort_unstable();
        if word_graphemes == anagram_graphemes {
            set.insert(anagram);
        }
    }
    set
}
