impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i = m - 1;
        let mut j = n - 1;
        let mut last = m + n - 1;

        while i >= 0 && j >= 0 {
            if nums1[i as usize] > nums2[j as usize] {
                nums1[last as usize] = nums1[i as usize];
                i -= 1;
            } else {
                nums1[last as usize] = nums2[j as usize];
                j -= 1;
            }
            last -= 1;
        }

        if j >= 0 {
            let count = (j + 1) as usize;
            nums1[..count].copy_from_slice(&nums2[..count]);
        }
    }
}
