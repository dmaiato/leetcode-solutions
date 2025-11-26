impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {

        for i in (0..digits.len()).rev() {
            if digits[i] == 9 {
                digits[i] = 0;
                continue;
            }
            digits[i] += 1;
            return digits;
        }
        digits = vec![0; digits.len() + 1];
        digits[0] += 1;
        return digits;
    }
}
