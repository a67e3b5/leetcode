/*
 * @lc app=leetcode id=345 lang=rust
 *
 * [345] Reverse Vowels of a String
 */

// @lc code=start
impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut i: usize = 0;
        let mut j: usize = s.chars().count() - 1;
        let mut output = s.into_bytes();
        let is_vowel = |b: &u8| {
            matches!(
                b,
                b'a' | b'e' | b'i' | b'o' | b'u' | b'A' | b'E' | b'I' | b'O' | b'U'
            )
        };
        while i < j {
            let head = output.get(i).unwrap();
            let tail = output.get(j).unwrap();
            let head_is_vowel = is_vowel(head);
            let tail_is_vowel = is_vowel(tail);

            if head_is_vowel && tail_is_vowel {
                output.swap(i, j);
                i += 1;
                j -= 1;
            } else if head_is_vowel {
                j -= 1;
            } else if tail_is_vowel {
                i += 1;
            } else {
                i += 1;
                j -= 1;
            }
        }
        String::from_utf8(output).unwrap()
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
