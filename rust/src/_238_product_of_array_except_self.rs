/*
 * @lc app=leetcode id=238 lang=rust
 *
 * [238] Product of Array Except Self
 */

// @lc code=start
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut ans = vec![1; len];
        let (mut l_acc, mut r_acc) = (1, 1);
        for i in 0..len {
            ans[i] *= l_acc;
            ans[len - 1 - i] *= r_acc;
            l_acc *= nums[i];
            r_acc *= nums[len - 1 - i];
        }
        ans
    }
}
// @lc code=end

struct Solution;
