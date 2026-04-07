//! [1768. Merge Strings Alternately](https://leetcode.com/problems/merge-strings-alternately/description/)

struct Solution;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        use itertools::Itertools;
        word1.chars().interleave(word2.chars()).collect()
    }
}

fn main() {
    let word1 = "abc".to_string();
    let word2 = "pqr".to_string();
    let res = Solution::merge_alternately(word1, word2);
    println!("{res}");
}
