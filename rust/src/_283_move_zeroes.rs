/*
 * @lc app=leetcode id=283 lang=rust
 *
 * [283] Move Zeroes
 */

// @lc code=start
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut zeros = 0;
        nums.retain(|n| {
            if *n == 0 {
                zeros += 1;
                false
            } else {
                true
            }
        });
        nums.append(&mut std::iter::repeat(0).take(zeros).collect());
    }
}
// @lc code=end

struct Solution;
