/*
 * @lc app=leetcode id=724 lang=rust
 *
 * [724] Find Pivot Index
 */

// @lc code=start
impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let nums_len = nums.len() as i32;

        if nums_len == 1 {
            return 0;
        }

        let mut l_num;
        let mut num = nums[0];
        let mut r_num = nums[1];

        let mut l_sum = 0;
        let mut r_sum = nums.iter().skip(1).sum::<i32>();

        if 0 == r_sum {
            return 0;
        }

        for (next_idx, next_num) in nums.iter().enumerate().skip(2) {
            l_num = num;
            num = r_num;
            r_num = *next_num;

            l_sum += l_num;
            r_sum -= num;

            if l_sum == r_sum {
                return next_idx as i32 - 1;
            }
        }

        l_sum += num;

        if l_sum == 0 {
            return nums_len - 1;
        }

        -1
    }
}
// @lc code=end

struct Solution;
