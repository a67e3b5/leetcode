/*
 * @lc app=leetcode id=3 lang=rust
 *
 * [3] Longest Substring Without Repeating Characters
 */

// @lc code=start
use std::cmp::max;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let n_chars = s.len();
        if n_chars < 2 {
            return n_chars as i32;
        }
        let s: Vec<char> = s.chars().collect();
        let mut l = 0;
        let mut r = 0;
        let mut max_len = 0;
        while r < n_chars {
            if let Some(i) = s[l..r].iter().position(|&c| c == s[r]) {
                max_len = max(max_len, r - l);
                l += i + 1;
                r += 1;
            } else {
                r += 1;
            }
        }
        max_len = max(max_len, r - l);
        max_len as i32
    }
}
// @lc code=end

struct Solution;
