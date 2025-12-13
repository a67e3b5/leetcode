/*
 * @lc app=leetcode id=53 lang=rust
 *
 * [53] Maximum Subarray
 */

// @lc code=start
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut cur = nums[0];
        let mut best = nums[0];

        for &x in &nums[1..] {
            cur = (cur + x).max(x);
            best = best.max(cur);
        }

        best
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test() {
        let cases = [
            (vec![-2, 1, -3, 4, -1, 2, 1, -5, 4], 6),
            (vec![1], 1),
            (vec![5, 4, -1, 7, 8], 23),
        ];
        for (args, ret) in cases {
            assert_eq!(
                super::Solution::max_sub_array(args.clone()),
                ret,
                "case: {args:?}, {ret:?}"
            )
        }
    }
}
