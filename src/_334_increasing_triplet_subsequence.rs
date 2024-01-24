/*
 * @lc app=leetcode id=334 lang=rust
 *
 * [334] Increasing Triplet Subsequence
 */

// @lc code=start
impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let (mut n0, mut n1) = (nums[0], i32::MAX);
        for num in nums {
            if num <= n0 {
                n0 = num;
            } else if num <= n1 {
                n1 = num;
            } else {
                return true;
            }
        }
        false
    }
}
// @lc code=end

struct Solution;
