/*
 * @lc app=leetcode id=217 lang=rust
 *
 * [217] Contains Duplicate
 */

// @lc code=start
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut seen = std::collections::HashSet::new();
        nums.into_iter().any(|n| !seen.insert(n))
    }
}
// @lc code=end

struct Solution;
