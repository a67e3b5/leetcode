/*
 * @lc app=leetcode id=9 lang=rust
 *
 * [9] Palindrome Number
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(mut x: i32) -> bool {
        if x.is_negative() {
            return false;
        }
        if x < 10 {
            return true;
        }
        let len: usize = (x as f32).log10().floor() as usize + 1;
        let mut digits = Vec::with_capacity(len / 2);
        for i in 0.. {
            if x == 0 {
                break;
            }
            if i == len / 2 && len % 2 == 1 {
                ()
            } else if i < len / 2 {
                digits.push((x % 10) as u8);
            } else {
                if (x % 10) as u8 != digits.pop().unwrap() {
                    return false;
                }
            }
            x /= 10;
        }
        true
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let cases = [
            (121, true),
            (-121, false),
            (10, false),
            (0, true),
            (1, true),
            (2, true),
            (11, true),
            (12, false),
            (123321, true),
            (1234321, true),
        ];
        for (args, ret) in cases {
            assert_eq!(
                super::Solution::is_palindrome(args),
                ret,
                "case: {args:?}, {ret:?}"
            )
        }
    }
}
