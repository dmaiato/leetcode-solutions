impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::new();
        }

        let first_word: &str = &strs[0];

        for i in 0..first_word.len() {
            let c = first_word.as_bytes()[i];
            
            for j in 1..strs.len() {
                let current_word = &strs[j];

                if i == current_word.len() || current_word.as_bytes()[i] != c {
                    return first_word[..i].to_string();
                }
            }
        }
        first_word.to_string()
    }
}
