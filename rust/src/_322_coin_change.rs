/*
 * @lc app=leetcode id=322 lang=rust
 *
 * [322] Coin Change
 */

// @lc code=start
use std::collections::VecDeque;

impl Solution {
    pub fn coin_change(mut coins: Vec<i32>, amount: i32) -> i32 {
        coins.sort_unstable_by_key(|c| -c);
        let mut queue = VecDeque::from([(amount, 0)]);
        while let Some((remain, num)) = queue.pop_front() {
            if remain == 0 {
                return num;
            }
            for c in &coins {
                let remain = remain - *c;
                if remain >= 0 {
                    queue.push_back((remain, num + 1));
                }
            }
        }
        -1
    }
}
// @lc code=end

struct Solution;
