use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let nums_map = nums
            .iter()
            .enumerate()
            .map(|(i, x)| (x, i))
            .collect::<HashMap<_, _>>();

        // It is important to iterate over the original array
        for (idx1, &num1) in nums.iter().enumerate() {
            let num2 = target - num1;
            let Some(&idx2) = nums_map.get(&num2) else {
                continue;
            };

            if num1 + num2 == target && idx1 != idx2 {
                return vec![idx1 as i32, idx2 as i32];
            }
        }

        unreachable!()
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn basic() {
        let nums = [3, 2, 4];
        let target = 6;

        let idxs = Solution::two_sum(nums.to_vec(), target);
        assert_eq!(nums[idxs[0] as usize] + nums[idxs[1] as usize], target);
    }
}
