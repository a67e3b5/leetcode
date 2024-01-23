/*
 * @lc app=leetcode id=338 lang=rust
 *
 * [338] Counting Bits
 */

// @lc code=start
impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut ans = vec![0, 1];
        let mut add = vec![1];
        while ans.len() < n + 1 {
            let mut tmp = vec![];
            for &num in &add {
                tmp.push(num);
                tmp.push(num + 1);
            }
            ans.extend(&tmp);
            add = tmp;
        }
        ans.truncate(n + 1);
        ans
    }
}
// @lc code=end

struct Solution;
