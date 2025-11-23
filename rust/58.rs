impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {

        return s.split_whitespace()
            .last()
            .map_or(0, |word| word.len() as i32);
        
        //let mut counter: i32 = 0;

        //for c in s.chars().rev() {
        //    if c == ' ' {
        //        if counter > 0 {
        //            return counter;
        //        }
        //        continue;
        //    }
        //    counter += 1;
        //}
        //return counter;
    }
}
