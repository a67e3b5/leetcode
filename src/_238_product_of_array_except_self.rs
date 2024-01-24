/*
 * @lc app=leetcode id=238 lang=rust
 *
 * [238] Product of Array Except Self
 */

// @lc code=start
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let l_arr = nums.iter().scan(1, |s, x| {
            let res = *s;
            *s *= x;
            Some(res)
        });
        let r_arr = nums
            .iter()
            .rev()
            .scan(1, |s, x| {
                let res = *s;
                *s *= x;
                Some(res)
            })
            .collect::<Vec<_>>()
            .into_iter()
            .rev();

        l_arr.zip(r_arr).map(|(l, r)| l * r).collect()
    }
}
// @lc code=end

struct Solution;
