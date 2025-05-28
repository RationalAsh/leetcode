use std::cmp::Reverse;

pub struct Solution {}

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        use std::collections::BinaryHeap;

        let mut heap = BinaryHeap::new();

        for i in 0..m {
            heap.push(Reverse(nums1[i as usize]));
        }

        for i in 0..n {
            heap.push(Reverse(nums2[i as usize]));
        }

        for i in 0..(m + n) {
            if let Some(val) = heap.pop() {
                nums1[i as usize] = val.0;
            }
        }
    }
}
