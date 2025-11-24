/*
 * @lc app=leetcode id=322 lang=rust
 *
 * [322] Coin Change
 */

// @lc code=start
impl Solution {
    pub fn coin_change(mut coins: Vec<i32>, amount: i32) -> i32 {
        coins.sort_unstable();
        let mut stack = vec![(amount, 0)];
        while let Some((remain, num)) = stack.pop() {
            if remain == 0 {
                return num;
            }
            for c in &coins {
                let remain = remain - *c;
                if remain >= 0 {
                    stack.push((remain, num + 1));
                }
            }
        }
        -1
    }
}
// @lc code=end

struct Solution;
