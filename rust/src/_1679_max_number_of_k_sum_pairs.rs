/*
 * @lc app=leetcode id=1679 lang=rust
 *
 * [1679] Max Number of K-Sum Pairs
 */

use std::usize;

// @lc code=start
impl Solution {
    pub fn max_operations(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        while let Some(end) = nums.pop() {
            let mut index_remove = usize::MAX;
            for (i, num) in nums.iter().enumerate() {
                if num + end == k {
                    index_remove = i;
                    break;
                }
            }
            if index_remove != usize::MAX {
                nums.remove(index_remove);
                count += 1;
            }
        }
        count
    }
}
// @lc code=end

struct Solution;
