/*
 * @lc app=leetcode id=322 lang=rust
 *
 * [322] Coin Change
 */

// @lc code=start
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let coins: Vec<usize> = coins.into_iter().map(|c| c as usize).collect();
        let amount = amount as usize;
        let mut dp = vec![i32::MAX; amount + 1];
        dp[0] = 0;
        for a in 1..=amount {
            let Some(min) = coins.iter().filter(|c| a >= **c).map(|c| dp[a - *c]).min() else {
                continue;
            };
            dp[a] = min.saturating_add(1);
        }
        if dp[amount] < i32::MAX {
            dp[amount]
        } else {
            -1
        }
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let cases = [(vec![2], 3, -1)];
        for (coins, amount, ret) in cases {
            assert_eq!(super::Solution::coin_change(coins, amount), ret)
        }
    }
}
