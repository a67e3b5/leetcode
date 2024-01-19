/*
 * @lc app=leetcode id=724 lang=rust
 *
 * [724] Find Pivot Index
 */

// @lc code=start
impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut l_sum = 0;
        let mut r_sum = nums.iter().sum::<i32>();

        #[allow(clippy::needless_range_loop)]
        for i in 0..nums.len() {
            r_sum -= nums[i];
            if l_sum == r_sum {
                return i as i32;
            }
            l_sum += nums[i];
        }
        -1
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
            (vec![1], 0),
            (vec![1, 1], -1),
            (vec![0, 1, -1, 0], 0),
            (vec![1, -1, 0], 2),
        ];
        for (src, res) in samples {
            assert_eq!(Solution::pivot_index(src), res)
        }
    }
}
