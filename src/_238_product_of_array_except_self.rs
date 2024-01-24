/*
 * @lc app=leetcode id=238 lang=rust
 *
 * [238] Product of Array Except Self
 */

// @lc code=start
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![1; nums.len()];
        for (i, num) in nums.into_iter().enumerate() {
            ans.iter_mut().enumerate().for_each(|(j, prod)| {
                if i != j {
                    *prod *= num
                }
            })
        }
        ans
    }
}
// @lc code=end

struct Solution;
