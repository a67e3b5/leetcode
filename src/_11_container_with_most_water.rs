/*
 * @lc app=leetcode id=11 lang=rust
 *
 * [11] Container With Most Water
 */

// @lc code=start
use std::cmp::{max, min};

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let height = height.into_iter().map(|h| h as usize).collect::<Vec<_>>();
        let area = |l_idx: usize, r_idx: usize| (r_idx - l_idx) * min(height[l_idx], height[r_idx]);
        let (mut l_idx, mut r_idx) = (0_usize, height.len() - 1);
        let mut ans = area(l_idx, r_idx);
        while l_idx < r_idx {
            let area_l_shrink = area(l_idx + 1, r_idx);
            let area_r_shrink = area(l_idx, r_idx - 1);
            dbg!(&l_idx, &r_idx, &ans, &area_l_shrink, &area_r_shrink);
            if ans < max(area_l_shrink, area_r_shrink) {
                if area_l_shrink < area_r_shrink {
                    ans = area_r_shrink;
                    r_idx -= 1;
                } else {
                    ans = area_l_shrink;
                    l_idx += 1;
                }
            } else {
                l_idx += 1;
                r_idx -= 1;
            }
        }
        ans as i32
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let samples = [(vec![2, 3, 10, 5, 7, 8, 9], 36)];
        for (input, output) in samples {
            assert_eq!(Solution::max_area(input), output);
        }
    }
}
