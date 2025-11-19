use std::collections::HashMap;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut map: HashMap<char, i32> = HashMap::new();
        map.insert('I', 1);
        map.insert('V', 5);
        map.insert('X', 10);
        map.insert('L', 50);
        map.insert('C', 100);
        map.insert('D', 500);
        map.insert('M', 1000);

        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        let mut total = 0;

        for i in 0..n {
            let current_value = map[&chars[i]];

            if i < n -1 {
                let next_value = map[&chars[i + 1]];

                if current_value < next_value {
                    total -= current_value;
                } else {
                    total += current_value;
                }
            } else {
                total += current_value;
            }
        }
        total
    }
}
