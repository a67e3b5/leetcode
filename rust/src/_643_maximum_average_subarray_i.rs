/*
 * @lc app=leetcode id=643 lang=rust
 *
 * [643] Maximum Average Subarray I
 */

// @lc code=start
impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let k = k as usize;
        let mut last_sum = nums[0..k].iter().sum::<i32>();
        let mut max_sum = last_sum;
        for offset in 0..(nums.len() - k) {
            last_sum = last_sum - nums[offset] + nums[offset + k];
            if max_sum < last_sum {
                max_sum = last_sum;
            }
        }
        max_sum as f64 / k as f64
    }
}
// @lc code=end

struct Solution;
