// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut h = HashMap::new();
    for word in magazine {
        *h.entry(word).or_insert(0) += 1;
    }
    for word in note {
        *h.entry(word).or_insert(0) -= 1;
    }
    h.values().all(|&x| x >= 0)
}
