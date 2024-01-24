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
            if n1 < num {
                return true;
            }
            if n0 < num {
                n1 = num;
            } else {
                n0 = num;
            }
        }
        false
    }
}
// @lc code=end

struct Solution;
