impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {

        return nums.partition_point(|&i| i < target) as i32; // binary search
        
        // for i in 0..nums.len() {
        //     if nums[i] >= target {
        //         return i as i32;
        //     }
        // }
        // return nums.len() as i32;
    }
}
