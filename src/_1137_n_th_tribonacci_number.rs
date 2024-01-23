/*
 * @lc app=leetcode id=1137 lang=rust
 *
 * [1137] N-th Tribonacci Number
 */

// @lc code=start
impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        match n {
            0 => 0,
            1 => 1,
            2 => 1,
            _ => Self::tribonacci(n - 3) + Self::tribonacci(n - 2) + Self::tribonacci(n - 1),
        }
    }
}
// @lc code=end

struct Solution;
