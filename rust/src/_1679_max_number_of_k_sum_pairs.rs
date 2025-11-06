/*
 * @lc app=leetcode id=1679 lang=rust
 *
 * [1679] Max Number of K-Sum Pairs
 */

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut res = 0;
        let mut hashmap = HashMap::new();
        for num in nums {
            let target = k - num;
            match hashmap.get_mut(&target) {
                Some(a) => {
                    if *a > 1 {
                        *a -= 1;
                    } else {
                        hashmap.remove_entry(&target);
                    }
                    res += 1;
                }
                None => {
                    *hashmap.entry(num).or_insert(0) += 1;
                }
            }
        }
        res
    }
}
// @lc code=end

struct Solution;
