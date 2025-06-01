pub struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let soln = nums.iter().fold((Vec::new(), 0), |(mut v, k), &num| {
            if num == val {
                (v, k + 1)
            } else {
                v.push(num);
                (v, k)
            }
        });

        for (idx, num) in soln.0.iter().enumerate() {
            nums[idx] = *num;
        }

        soln.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: run `remove_element`, keep only the first `k` results,
    /// sort them, and compare against the sorted expectation.
    fn check_case(mut nums: Vec<i32>, val: i32, expected: Vec<i32>) {
        let k = Solution::remove_element(&mut nums, val);
        assert_eq!(k as usize, expected.len(), "wrong k returned");

        let mut got = nums[..k as usize].to_vec();
        got.sort();
        let mut expect = expected.clone();
        expect.sort();

        assert_eq!(got, expect, "contents after removal differ");
    }

    // ───── LeetCode examples ────────────────────────────────────────────────────
    #[test]
    fn example_1() {
        check_case(vec![3, 2, 2, 3], 3, vec![2, 2]);
    }

    #[test]
    fn example_2() {
        check_case(vec![0, 1, 2, 2, 3, 0, 4, 2], 2, vec![0, 1, 4, 0, 3]);
    }

    // ───── Edge cases ──────────────────────────────────────────────────────────
    #[test]
    fn empty_input() {
        check_case(vec![], 1, vec![]);
    }

    #[test]
    fn all_elements_are_val() {
        check_case(vec![7, 7, 7], 7, vec![]);
    }

    #[test]
    fn no_elements_are_val() {
        check_case(vec![5, 6, 7], 4, vec![5, 6, 7]);
    }

    #[test]
    fn single_element_drop() {
        check_case(vec![9], 9, vec![]);
    }

    #[test]
    fn single_element_keep() {
        check_case(vec![9], 8, vec![9]);
    }

    #[test]
    fn alternating_pattern() {
        check_case(vec![1, 2, 1, 2, 1, 2], 1, vec![2, 2, 2]);
    }

    #[test]
    fn val_outside_nums_range() {
        // nums[i] ≤ 50 by constraint; use a bigger val to ensure nothing removed.
        check_case(vec![10, 20, 30], 60, vec![10, 20, 30]);
    }

    // ───── Stress / maximum-length check ───────────────────────────────────────
    #[test]
    fn max_length_many_removals() {
        // 100 elements: 0-49 twice. Remove all 25’s.
        let mut nums: Vec<i32> = (0..50).chain(0..50).collect();
        let expected: Vec<i32> = nums.iter().cloned().filter(|&x| x != 25).collect();
        check_case(nums, 25, expected);
    }
}
