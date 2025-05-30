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
