impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();

        for ch in s.chars() {
            match ch {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),

                ')' | ']' | '}' => {
                    if stack.pop() != Some(ch) {
                        return false;
                    }
                }
                _ => continue,
            }
        }
        stack.is_empty()
    }
}
