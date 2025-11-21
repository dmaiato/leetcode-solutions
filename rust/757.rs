use std::cmp::Ordering;

impl Solution {
    pub fn intersection_size_two(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_by(|x, y| {
            x[1].cmp(&y[1])
                .then_with(|| y[0].cmp(&x[0]))
        });

        let mut res: i32 = 0;
        
        let mut p1: i32 = -1;
        let mut p2: i32 = -1;

        for interval in intervals.iter() {
            let left = interval[0];
            let right = interval[1];

            if p2 < left {
                res += 2;
                p1 = right - 1;
                p2 = right;
            } 
            else if p1 < left {
                res += 1;

                if p2 == right {
                    p1 = right - 1;
                } else {
                    p1 = p2;
                    p2 = right;
                }
            }
        }
        res
    }
}
