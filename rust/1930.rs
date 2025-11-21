use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {

        let mut res: HashSet<(char, char)> = HashSet::new();
        let mut left: HashSet<char> = HashSet::new();
        let mut right: HashMap<char, i32> = HashMap::new();

        for c in s.chars() {
            *right.entry(c).or_insert(0) += 1;
        }

        for middle in s.chars() {

            if let Some(count) = right.get_mut(&middle) {
                *count -= 1;
            }

            for &outer in left.iter() {
                if let Some(&count) = right.get(&outer) {
                    if count > 0 {
                        res.insert((outer, middle));
                    }
                }
            }
            left.insert(middle);
        }
        res.len() as i32
    }
}
