/*
 * @lc app=leetcode id=1657 lang=rust
 *
 * [1657] Determine if Two Strings Are Close
 */

// @lc code=start
impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        let mut dict1 = vec![0; 26];
        let mut dict2 = vec![0; 26];

        for b in word1.into_bytes() {
            dict1[(b - b'a') as usize] += 1;
        }
        for b in word2.into_bytes() {
            dict2[(b - b'a') as usize] += 1;
        }

        let is_same_chars = dict1
            .iter()
            .zip(dict2.iter())
            .filter(|(&n1, &n2)| n1 * n2 == 0)
            .all(|(n1, n2)| n1 + n2 == 0);
        let is_same_occurs = {
            dict1.retain(|n| *n != 0);
            dict1.sort();
            dict2.retain(|n| *n != 0);
            dict2.sort();
            dict1 == dict2
        };

        is_same_chars && is_same_occurs
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let samples = [(("aaabbbbccddeeeeefffff", "aaaaabbcccdddeeeeffff"), false)];
        for (input, output) in samples {
            assert_eq!(
                Solution::close_strings(input.0.to_string(), input.1.to_string()),
                output
            );
        }
    }
}
