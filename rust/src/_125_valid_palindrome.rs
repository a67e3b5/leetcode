/*
 * @lc app=leetcode id=125 lang=rust
 *
 * [125] Valid Palindrome
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let mut l = 0;
        let mut r = s.len() - 1;
        while l < r {
            if !s[l].is_ascii_alphanumeric() {
                l += 1;
                continue;
            }
            if !s[r].is_ascii_alphanumeric() {
                r -= 1;
                continue;
            }
            if s[l].to_ascii_lowercase() != s[r].to_ascii_lowercase() {
                return false;
            }
            l += 1;
            r -= 1;
        }
        true
    }
}
// @lc code=end

struct Solution;
