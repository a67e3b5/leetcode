/*
 * @lc app=leetcode id=1657 lang=rust
 *
 * [1657] Determine if Two Strings Are Close
 */

// @lc code=start
impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        const OFFSET: usize = b'a' as _;
        let mut dict1 = [0; OFFSET + 26];
        let mut dict2 = [0; OFFSET + 26];

        for b in word1.into_bytes() {
            dict1[b as usize] += 1;
        }
        for b in word2.into_bytes() {
            dict2[b as usize] += 1;
        }

        let is_same_chars = dict1
            .iter()
            .zip(dict2.iter())
            .skip(OFFSET)
            .filter(|(&n1, &n2)| n1 * n2 == 0)
            .all(|(n1, n2)| n1 + n2 == 0);
        let is_same_occurs = {
            let mut occurs1 = dict1
                .into_iter()
                .skip(OFFSET)
                .filter(|&n| n != 0)
                .collect::<Vec<_>>();
            let mut occurs2 = dict2
                .into_iter()
                .skip(OFFSET)
                .filter(|&n| n != 0)
                .collect::<Vec<_>>();
            occurs1.sort();
            occurs2.sort();
            occurs1 == occurs2
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
