use std::collections::HashSet;

impl Solution {
    pub fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
        
        let mut copy = original;
        let values: HashSet<i32> = nums.into_iter().collect();

        loop {
            if values.contains(&copy) {
                copy *= 2;
                continue;
            }
            break;
        }

        copy
    }
}
