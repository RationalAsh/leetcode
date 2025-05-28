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

#[cfg(test)]
mod tests {
    use super::Solution;

    /// Helper that runs `Solution::merge` and checks the result.
    fn run_case(mut nums1: Vec<i32>, m: usize, nums2: Vec<i32>, n: usize, expected: Vec<i32>) {
        let mut nums2 = nums2.clone(); // LeetCode signature takes &mut Vec<i32>
        Solution::merge(&mut nums1, m as i32, &mut nums2, n as i32);
        assert_eq!(nums1, expected);
    }

    #[test] // Example 1
    fn merge_basic() {
        run_case(
            vec![1, 2, 3, 0, 0, 0],
            3,
            vec![2, 5, 6],
            3,
            vec![1, 2, 2, 3, 5, 6],
        );
    }

    #[test] // Example 2: nums2 empty
    fn merge_nums2_empty() {
        run_case(vec![1], 1, vec![], 0, vec![1]);
    }

    #[test] // Example 3: nums1 initially “empty”
    fn merge_nums1_empty() {
        run_case(vec![0], 0, vec![1], 1, vec![1]);
    }

    #[test] // All elements of nums1 precede nums2
    fn merge_no_overlap_front() {
        run_case(
            vec![1, 2, 3, 0, 0, 0],
            3,
            vec![4, 5, 6],
            3,
            vec![1, 2, 3, 4, 5, 6],
        );
    }

    #[test] // All elements of nums1 follow nums2
    fn merge_no_overlap_back() {
        run_case(
            vec![4, 5, 6, 0, 0, 0],
            3,
            vec![1, 2, 3],
            3,
            vec![1, 2, 3, 4, 5, 6],
        );
    }

    #[test] // Interleaved duplicates
    fn merge_with_duplicates() {
        run_case(
            vec![1, 1, 2, 0, 0, 0],
            3,
            vec![1, 2, 2],
            3,
            vec![1, 1, 1, 2, 2, 2],
        );
    }

    #[test] // Negative and positive values mixed
    fn merge_negatives_and_positives() {
        run_case(
            vec![-5, -3, -1, 0, 0, 0],
            3,
            vec![-4, -2, 2],
            3,
            vec![-5, -4, -3, -2, -1, 2],
        );
    }

    #[test] // Extreme bounds (min & max i32), length = 2 (smallest > 1 case)
    fn merge_extreme_values() {
        run_case(
            vec![i32::MIN, 0],
            1,
            vec![i32::MAX],
            1,
            vec![i32::MIN, i32::MAX],
        );
    }
}
