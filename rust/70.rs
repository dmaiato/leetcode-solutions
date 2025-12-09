impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        
        // fibonacci -> bottom up approach
        let (mut one, mut two) = (1, 1);

        for i in 0..n-1 {
            let temp = one;
            one += two;
            two = temp
        }
        one
    }
}
