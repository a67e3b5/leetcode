/*
 * @lc app=leetcode id=300 lang=rust
 *
 * [300] Longest Increasing Subsequence
 */

// @lc code=start
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![1; n];
        for j in 1..n {
            let Some(max) = dp[..j]
                .iter()
                .enumerate()
                .filter_map(|(i, len)| (nums[i] < nums[j]).then_some(*len))
                .max()
            else {
                continue;
            };
            dp[j] = max + 1;
        }
        *dp.iter().max().unwrap()
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let cases = [
            (vec![10, 9, 2, 5, 3, 7, 101, 18], 4),
            (vec![0, 1, 0, 3, 2, 3], 4),
            (vec![7, 7, 7, 7, 7, 7, 7], 1),
            (vec![1, 3, 6, 7, 9, 4, 10, 5, 6], 6),
        ];
        for (args, ret) in cases {
            assert_eq!(super::Solution::length_of_lis(args), ret)
        }
    }
}
