/*
 * @lc app=leetcode id=2215 lang=rust
 *
 * [2215] Find the Difference of Two Arrays
 */

// @lc code=start
impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::HashSet;
        let nums1 = nums1.into_iter().collect::<HashSet<_>>();
        let nums2 = nums2.into_iter().collect::<HashSet<_>>();
        let diff12 = nums1.difference(&nums2).cloned().collect::<Vec<_>>();
        let diff21 = nums2.difference(&nums1).cloned().collect::<Vec<_>>();
        vec![diff12, diff21]
    }
}
// @lc code=end

struct Solution;
