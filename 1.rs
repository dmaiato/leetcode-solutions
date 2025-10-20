use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;

            if let Some(&complement_index) = map.get(&complement) {
                return vec![complement_index, i as i32];
            }

            map.insert(num, i as i32);
        }
        return vec![];
    }
}
