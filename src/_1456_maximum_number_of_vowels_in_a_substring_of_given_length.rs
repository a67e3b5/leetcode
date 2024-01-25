/*
 * @lc app=leetcode id=1456 lang=rust
 *
 * [1456] Maximum Number of Vowels in a Substring of Given Length
 */

// @lc code=start
impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let s = s.into_bytes();
        let is_vowel = |b: &u8| matches!(b, b'a' | b'e' | b'i' | b'o' | b'u');
        s.windows(k as usize)
            .map(|slice| slice.iter().filter(|&b| is_vowel(b)).count())
            .max()
            .unwrap() as i32
    }
}
// @lc code=end

struct Solution;
