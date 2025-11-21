/*
 * @lc app=leetcode id=209 lang=rust
 *
 * [209] Minimum Size Subarray Sum
 */

// @lc code=start
use std::{
    cmp::{Ordering, min},
    i32,
};

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = 0;
        let mut sum = nums[0];
        let mut min_len = i32::MAX;
        while r < nums.len() {
            match sum.cmp(&target) {
                Ordering::Less => {
                    r += 1;
                    if r < nums.len() {
                        sum += nums[r];
                    }
                }
                Ordering::Greater | Ordering::Equal => {
                    min_len = min(min_len, (r - l + 1) as i32);
                    sum -= nums[l];
                    l += 1;
                }
            }
        }
        if min_len == i32::MAX { 0 } else { min_len }
    }
}
// @lc code=end

struct Solution;
