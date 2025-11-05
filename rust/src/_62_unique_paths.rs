/*
 * @lc app=leetcode id=62 lang=rust
 *
 * [62] Unique Paths
 */

// @lc code=start
use std::convert::TryInto;
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (m, n) = (m.min(n) as u128, m.max(n) as u128);
        if m == 1 {
            return 1;
        }
        ((n..(m + n - 1)).product::<u128>() / (1..m).product::<u128>())
            .try_into()
            .unwrap()
    }
}
// @lc code=end

struct Solution;
