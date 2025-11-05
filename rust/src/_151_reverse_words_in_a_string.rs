/*
 * @lc app=leetcode id=151 lang=rust
 *
 * [151] Reverse Words in a String
 */

// @lc code=start
impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_ascii_whitespace()
            .rev()
            .collect::<Vec<_>>()
            .join(" ")
    }
}
// @lc code=end

struct Solution;
