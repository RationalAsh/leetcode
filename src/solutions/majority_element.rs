pub struct Solution {}

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let mut freq_map: HashMap<i32, i32> = HashMap::new();

        for &n in nums.iter() {
            let entry = freq_map.entry(n).or_insert(0);
            *entry += 1;
        }

        for (&key, &entry) in freq_map.iter() {
            if entry as usize > (nums.len() / 2) {
                return key;
            }
        }

        0
    }
}
