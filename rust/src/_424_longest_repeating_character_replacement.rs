/*
 * @lc app=leetcode id=424 lang=rust
 *
 * [424] Longest Repeating Character Replacement
 */

// @lc code=start
use std::cmp::max;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut s: Vec<char> = s.chars().collect();
        let max_len = Self::character_replacement_inner(&s, k);
        s.reverse();
        let rmax_len = Self::character_replacement_inner(&s, k);
        max(max_len, rmax_len)
    }

    pub fn character_replacement_inner(s: &[char], k: i32) -> i32 {
        let mut l = 0;
        let mut r = 0;
        let mut c = s[0];
        let mut modify = k;
        let mut max_len = 0;
        while r < s.len() {
            if s[r] == c {
                r += 1;
            } else if modify > 0 {
                modify -= 1;
                r += 1;
            } else {
                max_len = max(max_len, r - l);
                l = r;
                c = s[l];
                modify = k;
            }
        }
        max_len = max(max_len, r - l);
        max_len as i32
    }
}
// @lc code=end

struct Solution;
