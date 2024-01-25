/*
 * @lc app=leetcode id=11 lang=rust
 *
 * [11] Container With Most Water
 */

// @lc code=start
use std::collections::VecDeque;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut height = height
            .into_iter()
            .map(|i| i as usize)
            .collect::<VecDeque<_>>();
        let mut ans = 0;
        while let Some(&h_l) = height.get(0) {
            let max_capacity = height
                .iter()
                .enumerate()
                .map(|(i, &h_r)| i * h_l.min(h_r))
                .max()
                .unwrap();
            if ans < max_capacity {
                ans = max_capacity;
            }
            height.pop_front();
        }
        ans as i32
    }
}
// @lc code=end

struct Solution;
