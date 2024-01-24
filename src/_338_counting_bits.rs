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
        let mut i = 0;
        while i <= n / 2 {
            ans[2 * i] = ans[i];
            if 2 * i < n {
                ans[2 * i + 1] = ans[i] + 1;
            }
            i += 1;
        }
        ans
    }
}
// @lc code=end

struct Solution;
