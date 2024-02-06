/*
 * @lc app=leetcode id=198 lang=rust
 *
 * [198] House Robber
 */

// @lc code=start
// This is the dual of #746
impl Solution {
    pub fn rob(loss: Vec<i32>) -> i32 {
        let mut street_end = loss.len();
        // Loss to the end from the end
        let mut l0 = 0;
        // Loss to the end from 1 step before the end
        let mut l1 = loss[street_end - 1];

        while 1 < street_end {
            let l2 = loss[street_end - 2] + l1.min(l0);
            street_end -= 1;
            l0 = l1;
            l1 = l2;
        }
        loss.into_iter().sum::<i32>() - l1.min(l0)
    }
}
// @lc code=end

struct Solution;
