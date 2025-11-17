/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

// @lc code=start
use std::collections::HashMap;

impl Solution {
    /// By trading a small amount of memory for a HashMap that stores previously seen numbers and their indices, we can solve the problem in O(n) time.
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen: HashMap<i32, usize> = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(&j) = seen.get(&complement) {
                return vec![j as i32, i as i32];
            } else {
                seen.insert(num, i);
            }
        }
        unreachable!()
    }
}
// @lc code=end

struct Solution;
