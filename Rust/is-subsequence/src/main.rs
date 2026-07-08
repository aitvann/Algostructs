#![allow(dead_code)]
#![allow(unused)]

use std::collections::HashMap;

struct Solution;

fn overcomplicated(s: &str, t: &str) -> bool {
    if s.is_empty() {
        return true;
    }

    let mut visited = HashMap::new();
    for (i, tc) in t.chars().enumerate() {
        for counter in visited.values_mut() {
            let next_char = *counter + 1;
            if tc == s.chars().nth(next_char).unwrap() {
                if next_char == s.len() - 1 {
                    return true;
                }

                *counter = next_char;
            }
        }

        if tc == s.chars().next().unwrap() {
            visited.insert(i, 0);

            if s.len() == 1 {
                return true;
            }
        }
    }

    false
}

fn simple(s: &str, t: &str) -> bool {
    if s.is_empty() {
        return true;
    }

    let mut s_iter = s.chars().peekable();
    for t_char in t.chars() {
        // Getting next character but not advancing an iterator just yet
        let Some(s_char) = s_iter.peek() else {
            return true;
        };

        if *s_char == t_char {
            // Will be checking the next character going forward
            _ = s_iter.next().unwrap();

            // Was that the last element?
            if s_iter.peek().is_none() {
                return true;
            }
        }
    }

    false
}

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        // overcomplicated(&s, &t)
        simple(&s, &t)
    }
}

fn main() {
    //
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leet_code10() {
        let s = "bb".to_string();
        let t = "ahbgdc".to_string();
        assert!(!Solution::is_subsequence(s, t));
    }

    #[test]
    fn leet_code11() {
        let s = "b".to_string();
        let t = "abc".to_string();
        assert!(Solution::is_subsequence(s, t));
    }

    #[test]
    fn basic() {
        let s = "hbg".to_string();
        let t = "ahbgdc".to_string();
        assert!(Solution::is_subsequence(s, t));
    }

    #[test]
    fn not_subsequence() {
        let s = "abc".to_string();
        let t = "ahbgdc".to_string();
        assert!(Solution::is_subsequence(s, t));
    }

    #[test]
    fn subsequence() {
        let s = "axc".to_string();
        let t = "ahbgdc".to_string();
        assert!(!Solution::is_subsequence(s, t));
    }
}
