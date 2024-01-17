/*
 * @lc app=leetcode id=345 lang=rust
 *
 * [345] Reverse Vowels of a String
 */

// @lc code=start
impl Solution {
    pub fn reverse_vowels(mut s: String) -> String {
        let vowel_indices = s
            .chars()
            .enumerate()
            .filter_map(|(i, c)| Self::is_vowel(c).then_some(i))
            .collect::<Vec<_>>();

        for (l_vowel_idx, r_vowel_idx) in vowel_indices
            .iter()
            .zip(vowel_indices.iter().rev())
            .take_while(|(l, r)| l < r)
        {
            let r_char = s.remove(*r_vowel_idx);
            let l_char = s.remove(*l_vowel_idx);
            s.insert(*l_vowel_idx, r_char);
            s.insert(*r_vowel_idx, l_char);
        }
        s
    }
    #[inline]
    fn is_vowel(c: char) -> bool {
        "AEIOUaeiou".contains(c)
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let samples = [
            ("Education", "odicatuEn"),
            ("Simultaneously", "Sumoltenauisly"),
            ("Authorize", "eithoruzA"),
            ("Miscellaneous", "Muscollenaeis"),
        ];
        for (src, res) in samples {
            assert_eq!(Solution::reverse_vowels(src.to_string()), res.to_string())
        }
    }
}
