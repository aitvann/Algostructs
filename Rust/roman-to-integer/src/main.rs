//! [13. Roman to Integer](https://leetcode.com/problems/roman-to-integer/description/)

struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut res = 0;
        let mut rest = s.as_str();

        macro_rules! roman_subtractive {
            ($prefix:literal, $value:literal) => {
                if let Some(stripped) = rest.strip_prefix($prefix) {
                    res += $value;
                    rest = stripped;
                    continue;
                }
            };
        }

        while !rest.is_empty() {
            roman_subtractive!("CM", 900);
            roman_subtractive!("CD", 400);
            roman_subtractive!("XC", 90);
            roman_subtractive!("XL", 40);
            roman_subtractive!("IX", 9);
            roman_subtractive!("IV", 4);
            roman_subtractive!("M", 1000);
            roman_subtractive!("D", 500);
            roman_subtractive!("C", 100);
            roman_subtractive!("L", 50);
            roman_subtractive!("X", 10);
            roman_subtractive!("V", 5);
            roman_subtractive!("I", 1);
        }

        res
    }
}

fn main() {
    let roman = "MCMXCIV".to_string();
    println!("roman: {roman}");
    let arabic = Solution::roman_to_int(roman);
    println!("arabic: {arabic}");
}
