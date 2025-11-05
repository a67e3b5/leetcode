/*
 * @lc app=leetcode id=1004 lang=rust
 *
 * [1004] Max Consecutive Ones III
 */

// @lc code=start
impl Solution {
    pub fn longest_ones(nums: Vec<i32>, mut k: i32) -> i32 {
        let arr_size = nums.len();
        let (mut l_idx, mut r_idx) = (0, 0);
        while r_idx < arr_size {
            if nums[r_idx] == 0 {
                k -= 1;
            }
            if k < 0 {
                if nums[l_idx] == 0 {
                    k += 1;
                }
                l_idx += 1;
            }
            r_idx += 1;
        }
        (r_idx - l_idx) as i32
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
            ((vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2), 6),
            (
                (
                    vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1],
                    3,
                ),
                10,
            ),
        ];
        for (input, output) in samples {
            assert_eq!(Solution::longest_ones(input.0, input.1), output);
        }
    }
}
