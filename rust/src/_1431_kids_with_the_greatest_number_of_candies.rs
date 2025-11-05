/*
 * @lc app=leetcode id=1431 lang=rust
 *
 * [1431] Kids With the Greatest Number of Candies
 */

// @lc code=start
impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let Some(max_candies) = candies.iter().max() else {
            return vec![];
        };
        candies
            .iter()
            .map(|i| i + extra_candies >= *max_candies)
            .collect()
    }
}
// @lc code=end

struct Solution;
