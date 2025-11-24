/*
 * @lc app=leetcode id=70 lang=rust
 *
 * [70] Climbing Stairs
 */

// @lc code=start
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        Self::one(n) + Self::two(n)
    }

    fn one(top: i32) -> i32 {
        match top {
            1 => 1,
            2 => 1,
            _ => Self::one(top - 1) + Self::two(top - 1),
        }
    }

    fn two(top: i32) -> i32 {
        match top {
            1 => 0,
            2 => 1,
            _ => Self::one(top - 2) + Self::two(top - 2),
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
