//! [2239. Find Closest Number to Zero](https://leetcode.com/problems/find-closest-number-to-zero/description/)

struct Solution;

impl Solution {
    pub fn find_closest_number(nums: Vec<i32>) -> i32 {
        use std::cmp;
        nums.into_iter()
            .min_by(|&num1, &num2| match num1.abs().cmp(&num2.abs()) {
                cmp::Ordering::Less => cmp::Ordering::Less,
                cmp::Ordering::Equal => cmp::Reverse(num1).cmp(&cmp::Reverse(num2)),
                cmp::Ordering::Greater => cmp::Ordering::Greater,
            })
            .unwrap()
    }
}

fn main() {
    let nums = vec![-4, -2, 1, 4, 8];
    let res = Solution::find_closest_number(nums);
    println!("{res}");
}
