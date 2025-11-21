/*
 * @lc app=leetcode id=283 lang=rust
 *
 * [283] Move Zeroes
 */

// @lc code=start
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let Some(mut r) = nums.iter().position(|&num| num == 0) else {
            return;
        };
        let mut w = r;
        while r < nums.len() {
            if nums[r] == 0 {
                r += 1;
                continue;
            }
            nums[w] = nums[r];
            r += 1;
            w += 1;
        }
        while w < nums.len() {
            nums[w] = 0;
            w += 1;
        }
    }
}
// @lc code=end

struct Solution;
