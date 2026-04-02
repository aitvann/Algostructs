#![allow(dead_code)]

mod update_efficient {
    pub struct NumArray {
        pub nums: Vec<i32>,
    }

    impl NumArray {
        pub fn new(nums: Vec<i32>) -> Self {
            Self { nums }
        }

        pub fn update(&mut self, index: i32, val: i32) {
            self.nums[index as usize] = val;
        }

        pub fn sum_range(&self, left: i32, right: i32) -> i32 {
            self.nums[left as usize..=right as usize].iter().sum()
        }
    }
}

mod sum_efficient {
    pub struct NumArray {
        pub nums: Vec<i32>,
        pub sums: Vec<i32>,
    }

    impl NumArray {
        pub fn new(nums: Vec<i32>) -> Self {
            let mut accumulator = 0;

            let sums = nums
                .iter()
                .map(|x| {
                    accumulator += x;
                    accumulator
                })
                .collect::<Vec<_>>();

            Self { nums, sums }
        }

        pub fn update(&mut self, index: i32, val: i32) {
            let index = index as usize;

            self.nums[index] = val;

            let mut accumulator = if index == 0 { 0 } else { self.sums[index - 1] };
            self.sums[index] = accumulator + val;

            for (x, sum) in self.nums[index..].iter().zip(self.sums[index..].iter_mut()) {
                accumulator += *x;
                *sum = accumulator;
            }
        }

        pub fn sum_range(&self, left: i32, right: i32) -> i32 {
            let left_idx = left as usize;
            let right_idx = right as usize;

            let prefix = if left_idx == 0 {
                0
            } else {
                self.sums[left_idx - 1]
            };

            self.sums[right_idx] - prefix
        }
    }
}

mod segment_tree {
    use std::iter;
    use std::ops;

    pub trait Aggregator: Default {
        fn aggregate(&self, other: &Self) -> Self;
    }

    #[derive(Debug)]
    pub struct SegmentTree<T> {
        data: Vec<T>,
        size: usize,
    }

    impl<T> Default for SegmentTree<T> {
        fn default() -> Self {
            Self {
                data: Default::default(),
                size: Default::default(),
            }
        }
    }

    impl<T: Aggregator> SegmentTree<T> {
        pub fn build_tree(mut source: impl ExactSizeIterator<Item = T>) -> Self {
            let tree_size = source.len() * 2;
            let mut data = iter::repeat_with(T::default)
                .take(tree_size)
                .collect::<Vec<_>>();

            let source_len = source.len();
            for x in &mut data[tree_size - source_len..] {
                *x = source.next().expect("same size");
            }

            let mut tree = Self {
                data,
                size: source_len,
            };

            for i in (1..=tree.size - 1).rev() {
                tree.data[i] = tree.child_left(i).aggregate(tree.child_right(i));
            }

            tree
        }

        pub fn query(&self, range: impl ops::RangeBounds<usize>) -> T {
            let ops::Range { start, end } = slice_range(range, ..self.size);
            let mut left_idx = start + self.size;
            let mut right_idx = end + self.size;

            let mut aggregator = T::default();
            while left_idx < right_idx {
                if left_idx % 2 == 1 {
                    aggregator = aggregator.aggregate(&self.data[left_idx]);
                    left_idx += 1;
                }

                if right_idx % 2 == 1 {
                    right_idx -= 1;
                    aggregator = aggregator.aggregate(&self.data[right_idx]);
                }

                left_idx /= 2;
                right_idx /= 2;
            }

            aggregator
        }

        pub fn update(&mut self, index: usize, value: T) {
            let mut index = index + self.size;
            self.data[index] = value;

            while index > 1 {
                index /= 2;
                self.data[index] = self.child_left(index).aggregate(self.child_right(index));
            }
        }

        fn parent(&self, index: usize) -> &T {
            &self.data[index / 2]
        }

        fn child_left(&self, index: usize) -> &T {
            &self.data[index * 2]
        }

        fn child_right(&self, index: usize) -> &T {
            &self.data[index * 2 + 1]
        }
    }

    fn slice_range<R>(range: R, bounds: ops::RangeTo<usize>) -> ops::Range<usize>
    where
        R: ops::RangeBounds<usize>,
    {
        let len = bounds.end;

        let end = match range.end_bound() {
            ops::Bound::Included(&end) if end >= len => panic!("out of bound"),
            // Cannot overflow because `end < len` implies `end < usize::MAX`.
            ops::Bound::Included(&end) => end + 1,

            ops::Bound::Excluded(&end) if end > len => panic!("out of bound"),
            ops::Bound::Excluded(&end) => end,
            ops::Bound::Unbounded => len,
        };

        let start = match range.start_bound() {
            ops::Bound::Excluded(&start) if start >= end => panic!("out of bound"),
            // Cannot overflow because `start < end` implies `start < usize::MAX`.
            ops::Bound::Excluded(&start) => start + 1,

            ops::Bound::Included(&start) if start > end => panic!("out of bound"),
            ops::Bound::Included(&start) => start,

            ops::Bound::Unbounded => 0,
        };

        ops::Range { start, end }
    }
}

mod universally_efficient {
    use std::ops::Add;

    use super::segment_tree::*;

    #[derive(Clone, Default, Debug)]
    pub struct Sum<T>(pub T);

    impl<T> Aggregator for Sum<T>
    where
        for<'a> &'a T: Add<&'a T, Output = T>,
        T: Default,
    {
        fn aggregate(&self, other: &Self) -> Self {
            Self(&self.0 + &other.0)
        }
    }

    pub struct NumArray {
        inner: SegmentTree<Sum<i32>>,
    }

    impl NumArray {
        pub fn new(nums: Vec<i32>) -> Self {
            Self {
                inner: SegmentTree::build_tree(nums.into_iter().map(Sum)),
            }
        }

        pub fn update(&mut self, index: i32, val: i32) {
            self.inner.update(index as usize, Sum(val));
        }

        pub fn sum_range(&self, left: i32, right: i32) -> i32 {
            self.inner.query(left as usize..=right as usize).0
        }
    }
}

// use update_efficient::*;
// use sum_efficient::*;
use universally_efficient::*;

fn main() {
    let nums = vec![5, 19, 0, 1, 3, 43];
    let mut num_array = NumArray::new(nums);
    println!("Sum: {}", num_array.sum_range(1, 4));
    num_array.update(2, 7);
    println!("Sum: {}", num_array.sum_range(1, 4));
}

#[cfg(test)]
mod tests {
    #[test]
    fn foo() {
        dbg!(());
    }
}
