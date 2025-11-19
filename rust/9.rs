impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        if x % 10 == 0 && x != 0 {
            return false;
        }

        if x == 0  {
            return true;
        }

        let mut copy = x;

        let mut reversed = 0;
        while copy != 0 {
            let last_digit = copy % 10;
            reversed = reversed *10 + last_digit;
            copy /= 10;
        }

        if x / reversed == 1 {
            return true;
        } else {
            return false;
        }
    }
}
