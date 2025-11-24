/*
 * @lc app=leetcode id=198 lang=rust
 *
 * [198] House Robber
 */

// @lc code=start
use std::cmp::max;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut no = 0;
        let mut yes = nums[0];
        for i in 1..nums.len() {
            let tmp = no;
            no = max(no, yes);
            yes = max(yes, tmp + nums[i]);
        }
        max(no, yes)
    }
}
// @lc code=end

struct Solution;
