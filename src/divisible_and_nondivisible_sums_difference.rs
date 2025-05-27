//! You are given positive integers n and m.

//! Define two integers as follows:

//! num1: The sum of all integers in the range [1, n] (both inclusive) that are not divisible by m.
//! num2: The sum of all integers in the range [1, n] (both inclusive) that are divisible by m.
//! Return the integer num1 - num2.

//! Example 1:

//! Input: n = 10, m = 3
//! Output: 19
//! Explanation: In the given example:
//! - Integers in the range [1, 10] that are not divisible by 3 are [1,2,4,5,7,8,10], num1 is the sum of those integers = 37.
//! - Integers in the range [1, 10] that are divisible by 3 are [3,6,9], num2 is the sum of those integers = 18.
//! We return 37 - 18 = 19 as the answer.
//! Example 2:

//! Input: n = 5, m = 6
//! Output: 15
//! Explanation: In the given example:
//! - Integers in the range [1, 5] that are not divisible by 6 are [1,2,3,4,5], num1 is the sum of those integers = 15.
//! - Integers in the range [1, 5] that are divisible by 6 are [], num2 is the sum of those integers = 0.
//! We return 15 - 0 = 15 as the answer.
//! Example 3:

//! Input: n = 5, m = 1
//! Output: -15
//! Explanation: In the given example:
//! - Integers in the range [1, 5] that are not divisible by 1 are [], num1 is the sum of those integers = 0.
//! - Integers in the range [1, 5] that are divisible by 1 are [1,2,3,4,5], num2 is the sum of those integers = 15.
//! We return 0 - 15 = -15 as the answer.

//! Constraints:

//! 1 <= n, m <= 1000
pub struct Solution {}

impl Solution {
    pub fn difference_of_sums(n: i32, m: i32) -> i32 {
        let (num1, num2) = (1..(n + 1)).fold((0, 0), |(num1, num2), i| {
            if i % m == 0 {
                (num1, num2 + i)
            } else {
                (num1 + i, num2)
            }
        });

        num1 - num2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ───────────────────────────────────
    // Basic examples from the prompt
    // ───────────────────────────────────
    #[test]
    fn example_1() {
        assert_eq!(Solution::difference_of_sums(10, 3), 19);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::difference_of_sums(5, 6), 15);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::difference_of_sums(5, 1), -15);
    }

    // ───────────────────────────────────
    // Edge-case coverage
    // ───────────────────────────────────
    /// n and m both at the lower bound (1).
    /// All numbers are divisible → result is −1.
    #[test]
    fn lower_bound_all_divisible() {
        assert_eq!(Solution::difference_of_sums(1, 1), -1);
    }

    /// n = 1, m > n.
    /// Nothing divisible → result is +1.
    #[test]
    fn lower_bound_none_divisible() {
        assert_eq!(Solution::difference_of_sums(1, 2), 1);
    }

    /// m > n (no multiples), moderate size.
    #[test]
    fn no_multiples_in_range() {
        assert_eq!(Solution::difference_of_sums(7, 10), 28);
    }

    /// m = n (exactly one multiple at the end of the range).
    #[test]
    fn single_multiple_at_end() {
        assert_eq!(Solution::difference_of_sums(8, 8), 20);
    }

    // ───────────────────────────────────
    // Upper-limit stress tests (n = 1000)
    // ───────────────────────────────────
    /// Every number is divisible (m = 1) → strongly negative result.
    #[test]
    fn all_divisible_large() {
        assert_eq!(Solution::difference_of_sums(1000, 1), -500_500);
    }

    /// Exactly half the numbers are divisible (m = 2) → small negative result.
    #[test]
    fn half_divisible_large() {
        assert_eq!(Solution::difference_of_sums(1000, 2), -500);
    }

    /// Only one number (1000) is divisible → large positive result.
    #[test]
    fn single_divisible_large() {
        assert_eq!(Solution::difference_of_sums(1000, 1000), 498_500);
    }
}
