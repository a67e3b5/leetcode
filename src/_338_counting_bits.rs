/*
 * @lc app=leetcode id=338 lang=rust
 *
 * [338] Counting Bits
 */

// @lc code=start
impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut ans = vec![0; n + 1];
        for i in 1..=n {
            ans[i] = ans[i >> 1] + (i & 1) as i32;
        }
        ans
    }
}
// @lc code=end

struct Solution;
