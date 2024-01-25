/*
 * @lc app=leetcode id=1456 lang=rust
 *
 * [1456] Maximum Number of Vowels in a Substring of Given Length
 */

// @lc code=start
impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let k = k as usize;
        let is_vowel = |b: &u8| matches!(b, b'a' | b'e' | b'i' | b'o' | b'u');
        let booleans = s.into_bytes().iter().map(is_vowel).collect::<Vec<_>>();
        let mut num_vowels = booleans.iter().take(k).filter(|&&b| b).count();
        let mut ans = num_vowels;
        let mut i = 0;
        while i + k < booleans.len() {
            let (excluding, including) = (booleans[i], booleans[i + k]);
            if excluding ^ including {
                if excluding {
                    num_vowels -= 1;
                } else {
                    num_vowels += 1;
                    if ans < num_vowels {
                        ans = num_vowels
                    }
                }
            }
            i += 1;
        }
        ans as i32
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let samples = [(("abciiidef", 3), 3)];
        for (input, output) in samples {
            assert_eq!(Solution::max_vowels(input.0.to_string(), input.1), output);
        }
    }
}
