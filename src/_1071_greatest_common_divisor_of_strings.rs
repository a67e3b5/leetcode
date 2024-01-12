/*
 * @lc app=leetcode id=1071 lang=rust
 *
 * [1071] Greatest Common Divisor of Strings
 */

// @lc code=start
impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let mut dividend = &*str1;
        let mut divisor = &*str2;
        let mut can_be_mutually_prime = false;
        loop {
            let remainder = dividend.trim_start_matches(divisor);
            if remainder == dividend {
                if can_be_mutually_prime {
                    break String::new();
                }
                can_be_mutually_prime = true;
            } else {
                can_be_mutually_prime = false;
            }
            if remainder.is_empty() {
                break divisor.to_string();
            }
            dividend = divisor;
            divisor = remainder;
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
        let str1 = "QwerQwer".to_string();
        let str2 = "QwerQwerQwer".to_string();
        let result = Solution::gcd_of_strings(str1, str2);
        let expected = "Qwer".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn str2_is_prime() {
        let str1 = "QwerQwer".to_string();
        let str2 = "QwerQwerQ".to_string();
        let result = Solution::gcd_of_strings(str1, str2);
        let expected = "".to_string();
        assert_eq!(result, expected);
    }
}
