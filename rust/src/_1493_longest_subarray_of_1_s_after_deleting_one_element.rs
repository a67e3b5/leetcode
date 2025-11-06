/*
 * @lc app=leetcode id=1493 lang=rust
 *
 * [1493] Longest Subarray of 1's After Deleting One Element
 */

// @lc code=start
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let Some(mut l_idx) = nums[0..].iter().position(|n| *n == 1) else {
            return 0;
        };
        let Some(mut zero_idx) = nums[l_idx..].iter().position(|n| *n == 0) else {
            return (len - l_idx) as i32;
        };
        let Some(mut r_idx) = nums[zero_idx..]
            .iter()
            .position(|n| *n == 0)
            .map(|second_zero_idx| second_zero_idx - 1)
        else {
            return (len - l_idx) as i32;
        };
        let mut len_ones = r_idx - l_idx;
        while r_idx + 1 < len {
            if let Some(l_idx_new) = nums[r_idx + 1..].iter().position(|n| *n == 1) {
                l_idx = l_idx_new
            } else {
                break;
            };
            if let Some(zero_idx_new) = nums[l_idx..].iter().position(|n| *n == 0) {
                zero_idx = zero_idx_new
            } else {
                len_ones = len_ones.max(len - l_idx);
                break;
            };
            if let Some(r_idx_new) = nums[zero_idx..]
                .iter()
                .position(|n| *n == 0)
                .map(|second_zero_idx| second_zero_idx - 1)
            {
                r_idx = r_idx_new
            } else {
                len_ones = len_ones.max(len - l_idx);
                break;
            };
            len_ones = len_ones.max(r_idx - l_idx);
        }
        len_ones as i32
    }
}
// @lc code=end

struct Solution;

mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::longest_subarray(vec![1, 1, 0, 1]), 4);
        assert_eq!(Solution::longest_subarray(vec![0, 0, 0, 0, 0]), 0);
        assert_eq!(Solution::longest_subarray(vec![0, 1, 1, 1, 1]), 4);
        assert_eq!(Solution::longest_subarray(vec![1, 1, 1, 1, 1]), 4);
        assert_eq!(Solution::longest_subarray(vec![0, 1, 1, 1, 0]), 3);
        assert_eq!(Solution::longest_subarray(vec![1, 1, 1, 1, 0]), 4);
        assert_eq!(Solution::longest_subarray(vec![0, 1, 0, 1, 0]), 3);
        assert_eq!(Solution::longest_subarray(vec![1, 0, 1, 0, 1]), 3);
        assert_eq!(Solution::longest_subarray(vec![0, 1, 0, 0, 1]), 1);
    }
}
