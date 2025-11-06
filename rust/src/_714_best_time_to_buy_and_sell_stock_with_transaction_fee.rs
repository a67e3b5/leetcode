/*
 * @lc app=leetcode id=714 lang=rust
 *
 * [714] Best Time to Buy and Sell Stock with Transaction Fee
 */

// @lc code=start
impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let len = prices.len();
        let mut hold = 0 - prices[0];
        let mut free = 0;
        for i in 0..len {
            let prev_hold = hold;
            hold = hold.max(free - prices[i]);
            free = free.max(prev_hold + prices[i] - fee);
        }
        free
    }
}
// @lc code=end

struct Solution;
