#[derive(Debug)]
pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        solution(nums, target)
    }
}

fn solution(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;

    let mut map: HashMap<i32, usize> = HashMap::new();

    // First insert the number's index into the hashmap.
    for (idx, num) in nums.iter().enumerate() {
        map.insert(num.clone(), idx);
    }

    // Then loop over each number
    for (_idx, num) in nums.iter().enumerate() {
        let q = target - num;
        match map.get(&q) {
            Some(idx) => {
                // If the index exists and is not equal
                if *idx != _idx {
                    return vec![idx.clone() as i32, _idx as i32];
                }
            }
            None => {}
        }
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;

        let solution = Solution::two_sum(nums, target);

        let s0 = solution[0];
        let s1 = solution[1];

        assert!(((s0 == 0) && (s1 == 1)) || ((s0 == 1) && (s1 == 0)));
    }

    #[test]
    fn test_2() {
        let nums = vec![3, 2, 4];
        let target = 6;

        let solution = Solution::two_sum(nums, target);

        let s0 = solution[0];
        let s1 = solution[1];

        let a0 = 1;
        let a1 = 2;

        assert!(((s0 == a0) && (s1 == a1)) || ((s0 == a1) && (s1 == a0)));
    }

    #[test]
    fn test_3() {
        let nums = vec![3, 3];
        let target = 6;

        let solution = Solution::two_sum(nums, target);

        let s0 = solution[0];
        let s1 = solution[1];

        let a0 = 1;
        let a1 = 0;

        assert!(((s0 == a0) && (s1 == a1)) || ((s0 == a1) && (s1 == a0)));
    }
}
