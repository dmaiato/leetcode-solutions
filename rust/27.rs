impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        
        let mut write_index: usize = 0;

        for i in 0..nums.len() {
            if nums[i] != val {
                nums[write_index] = nums[i];
                write_index += 1;
            }
        }
        write_index as i32
    }
}
