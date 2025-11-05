/*
 * @lc app=leetcode id=1137 lang=rust
 *
 * [1137] N-th Tribonacci Number
 */

// @lc code=start
impl Solution {
    pub fn tribonacci(mut n: i32) -> i32 {
        let (mut t0, mut t1, mut t2) = (0, 1, 1);
        while n > 0 {
            let t3 = t0 + t1 + t2;
            t0 = t1;
            t1 = t2;
            t2 = t3;
            n -= 1;
        }
        t0
    }
}
// @lc code=end

struct Solution;
