/*
 * @lc app=leetcode id=33 lang=rust
 *
 * [33] Search in Rotated Sorted Array
 */

// @lc code=start
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut d = nums.len() - 1;
        let mut i = 0;
        loop {
            if nums[i] < target {
                i += d;
                i %= nums.len();
            } else if nums[i] > target {
                if i < d {
                    i += nums.len();
                }
                i -= d;
            } else {
                return i as i32;
            }
            if d == 0 {
                break
            }
            d /= 2;
        }
        -1
    }
}
// @lc code=end

struct Solution;
