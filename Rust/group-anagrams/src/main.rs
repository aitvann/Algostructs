#![allow(unused)]

use std::collections::{BTreeMap, HashMap, HashSet};
use std::hash::{self, Hash};

/// This works but slow because of sorting
/// The straightforward way would be to use `HashMap<char, u32>`
/// instead of sorted arrays but `HashMap` does not implement `Hash`.
/// `BTreeMap` would also work but it's even slower for some reason.
/// So instead use `[0; 26]` where each slot is a letter of English alphabet
struct Solution;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut visited = HashMap::<_, Vec<String>>::new();
        for word in strs {
            let mut characters = BTreeMap::new();
            for char in word.chars() {
                characters
                    .entry(char)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }

            use std::collections::hash_map::Entry;
            match visited.entry(characters) {
                Entry::Occupied(mut entry) => {
                    entry.get_mut().push(word);
                }
                Entry::Vacant(entry) => {
                    entry.insert(vec![word]);
                }
            }
        }

        visited.into_values().collect()
    }
}

fn main() {
    println!("Hello, world!");
}

// TODO: replace `assert_eq` with something more sophisticate
#[cfg(test)]
mod tests {
    use std::fmt::Display;

    use crate::Solution;

    fn input(values: &[&str]) -> Vec<String> {
        values.iter().map(ToString::to_string).collect()
    }

    fn output(values: &[Vec<&str>]) -> Vec<Vec<String>> {
        values
            .iter()
            .map(|xs| xs.iter().map(|&x| x.to_owned()).collect::<Vec<_>>())
            .collect::<Vec<_>>()
    }

    #[test]
    fn basic() {
        let strs = input(&["eat", "tea", "tan", "ate", "nat", "bat"]);
        let expected = output(&[vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]]);
        let result = Solution::group_anagrams(strs);
        assert_eq!(result, expected);
    }

    #[test]
    fn leet_code108() {
        let strs = input(&["ddddddddddg", "dgggggggggg"]);
        let expected = output(&[vec!["dgggggggggg"], vec!["ddddddddddg"]]);
        let result = Solution::group_anagrams(strs);
        assert_eq!(result, expected);
    }
}
