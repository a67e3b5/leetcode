/*
 * @lc app=leetcode id=12 lang=rust
 *
 * [12] Integer to Roman
 */

// @lc code=start

impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let mut ans = String::new();
        while num > 0 {
            macro_rules! push_if {
                ($threshold:literal, $symbol:literal) => {
                    if num >= $threshold {
                        ans.push_str($symbol);
                        num -= $threshold;
                        continue;
                    }
                };
            }
            push_if!(1000, "M");
            push_if!(900, "CM");
            push_if!(500, "D");
            push_if!(400, "CD");
            push_if!(100, "C");
            push_if!(90, "XC");
            push_if!(50, "L");
            push_if!(40, "XL");
            push_if!(10, "X");
            push_if!(9, "IX");
            push_if!(5, "V");
            push_if!(4, "IV");
            push_if!(1, "I");
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
        let cases = [
            (3749, "MMMDCCXLIX".to_string()),
            (58, "LVIII".to_string()),
            (1994, "MCMXCIV".to_string()),
        ];
        for (args, ret) in cases {
            assert_eq!(
                super::Solution::int_to_roman(args),
                ret,
                "case: {args:?}, {ret:?}"
            )
        }
    }
}
