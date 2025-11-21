/*
 * @lc app=leetcode id=11 lang=rust
 *
 * [11] Container With Most Water
 */

// @lc code=start
use std::cmp::max;
use std::cmp::min;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = height.len() - 1;
        let mut h = min(height[l], height[r]);
        let mut area = h * (r - l) as i32;
        while l < r {
            if height[l] <= h {
                l += 1;
                continue;
            }
            if height[r] <= h {
                r -= 1;
                continue;
            }
            h = min(height[l], height[r]);
            area = max(area, h * (r - l) as i32);
        }
        area
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let samples = [
            (vec![2, 3, 10, 5, 7, 8, 9], 36),
            (vec![1, 3, 2, 5, 25, 24, 5], 24),
        ];
        for (input, output) in samples {
            assert_eq!(Solution::max_area(input), output);
        }
    }
}
