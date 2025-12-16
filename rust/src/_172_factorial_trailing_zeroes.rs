/*
 * @lc app=leetcode id=172 lang=rust
 *
 * [172] Factorial Trailing Zeroes
 */

// @lc code=start
impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let mut ans = 0;
        for mut i in 1..=(n / 5) {
            while i % 5 == 0 {
                ans += 1;
                i /= 5;
            }
            ans += 1;
        }
        ans
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let cases = [(3, 0), (5, 1), (0, 0), (10, 2), (15, 3), (20, 4), (25, 6), (125, 31)];
        for (args, ret) in cases {
            assert_eq!(
                super::Solution::trailing_zeroes(args),
                ret,
                "case: {args:?}, {ret:?}"
            )
        }
    }
}
