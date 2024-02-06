/*
 * @lc app=leetcode id=198 lang=rust
 *
 * [198] House Robber
 */

// @lc code=start
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .fold((0, 0), |(sum2, sum1), num0| (sum1, sum1.max(sum2 + num0)))
            .1
    }
}
// @lc code=end

struct Solution;
