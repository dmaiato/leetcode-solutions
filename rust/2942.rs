impl Solution {
    pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
        let mut results: Vec<i32> = Vec::new();

        for (i, word) in words.iter().enumerate() {
            if word.contains(x) {
                results.push(i as i32);
            }
        }
        return results;
    }
}
