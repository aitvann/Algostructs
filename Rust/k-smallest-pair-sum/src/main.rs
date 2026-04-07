#![allow(dead_code)]

mod naive_and_wrong_solution {
    pub fn k_smallest_pairs(nums1: &[i32], nums2: &[i32], k: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut i = 0;
        let mut j = 0;

        while res.len() < k as usize {
            res.push(vec![nums1[i], nums2[j]]);

            let next_i = i + 1;
            let next_j = j + 1;

            if next_i >= nums1.len() {
                break;
            }
            if next_j >= nums2.len() {
                break;
            }

            if nums1[next_i] < nums2[next_j] {
                i = next_i;
            } else {
                j = next_j;
            }
        }

        res
    }
}

mod brute {
    use itertools::Itertools;

    pub fn k_smallest_pairs(nums1: &[i32], nums2: &[i32], k: i32) -> Vec<Vec<i32>> {
        let mut sums = nums1.iter().cartesian_product(nums2).collect_vec();
        sums.sort_by_key(|&(a1, b1)| a1 + b1);
        sums.iter()
            .take(k as usize)
            .map(|&(&a, &b)| vec![a, b])
            .collect()
    }
}

mod heap {
    use std::{cmp, collections::BinaryHeap};

    pub fn k_smallest_pairs(nums1: &[i32], nums2: &[i32], k: i32) -> Vec<Vec<i32>> {
        let mut heap = nums1
            .iter()
            .enumerate()
            .map(|(i, &num1)| (num1 + nums2[0], i, 0))
            .map(cmp::Reverse)
            .collect::<BinaryHeap<_>>();

        let mut res = vec![];
        while res.len() < k as usize {
            if let Some(cmp::Reverse((_sum, i, j))) = heap.pop() {
                res.push(vec![nums1[i], nums2[j]]);

                let next_j = j + 1;
                if next_j < nums2.len() {
                    heap.push(cmp::Reverse((nums1[i] + nums2[next_j], i, next_j)));
                }
            }
        }

        res
    }
}

// use naive_and_wrong_solution::*;
// use brute::*;
use heap::*;

struct Solution;

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        k_smallest_pairs(&nums1, &nums2, k)
    }
}

fn main() {
    let nums1 = vec![1, 2, 4, 5, 6];
    let nums2 = vec![3, 5, 7, 9];
    let k = 3;

    let res = Solution::k_smallest_pairs(nums1, nums2, k);
    println!("{res:?}");
}
