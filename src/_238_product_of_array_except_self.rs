/*
 * @lc app=leetcode id=238 lang=rust
 *
 * [238] Product of Array Except Self
 */

// @lc code=start
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0; nums.len()];
        for (i, a) in ans.iter_mut().enumerate() {
            *a = nums.iter().enumerate().fold(
                1,
                |acc, (j, &num)| {
                    if i != j {
                        acc * num
                    } else {
                        acc
                    }
                },
            )
        }
        ans
    }
}
// @lc code=end

struct Solution;
