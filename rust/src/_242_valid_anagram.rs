/*
 * @lc app=leetcode id=242 lang=rust
 *
 * [242] Valid Anagram
 */

// @lc code=start
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let freq = |s: String| {
            let mut arr = [0; 26];
            s.chars().for_each(|c| arr[(c as u8 - b'a') as usize] += 1);
            arr
        };
        freq(s) == freq(t)
    }
}
// @lc code=end

struct Solution;
