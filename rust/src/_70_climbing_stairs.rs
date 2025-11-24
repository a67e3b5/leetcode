/*
 * @lc app=leetcode id=70 lang=rust
 *
 * [70] Climbing Stairs
 */

// @lc code=start
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut last = 0;
        let mut curr = 1;
        for _ in 0..n {
            let tmp = curr;
            curr = last + curr;
            last = tmp;
        }
        curr
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let cases = [(2, 2), (3, 3)];
        for (args, ret) in cases {
            assert_eq!(
                super::Solution::climb_stairs(args),
                ret,
                "case: {args}, {ret}"
            )
        }
    }
}
