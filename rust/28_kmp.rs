impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
       
        let nl = needle.len();
        let hl = haystack.len();

        let mut lps: Vec<usize> = vec![0; nl];
        let mut pre: usize = 0;

        let n_bytes = needle.as_bytes();
        let h_bytes = haystack.as_bytes();

        // lps preprocessing
        for i in 1..nl {
            while pre > 0 && n_bytes[i] != n_bytes[pre] {
                pre = lps[pre - 1];
            }
            if n_bytes[pre] == n_bytes[i] {
                pre += 1;
                lps[i] = pre;
            }
        }
        
        let mut ni: usize = 0; // needle_index

        // finding the match
        for hi in 0..hl {
            while ni > 0 && n_bytes[ni] != h_bytes[hi] {
                ni = lps[ni - 1];
            }
            if n_bytes[ni] == h_bytes[hi] {
                ni += 1;
            }
            if ni == nl {
                return (hi - ni + 1) as i32;
            }
        }

        -1
    }
}
