/*
 * @lc app=leetcode id=714 lang=rust
 *
 * [714] Best Time to Buy and Sell Stock with Transaction Fee
 */

// @lc code=start
impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let len = prices.len();
        let mut matrix = vec![vec![0; len]; len];
        for i in 0..len {
            for j in i + 1..len {
                matrix[i][j] = prices[j] - prices[i] - fee;
            }
        }
        let mut profits = vec![0; len];
        for j in 0..len {
            profits[j] = (0..=j)
                .map(|i| {
                    let profit = if i == 0 { 0 } else { profits[i - 1] };
                    profit + matrix[i][j]
                })
                .max()
                .unwrap();
        }
        profits[len - 1]
    }
}
// @lc code=end

struct Solution;
