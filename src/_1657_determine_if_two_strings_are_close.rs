/*
 * @lc app=leetcode id=1657 lang=rust
 *
 * [1657] Determine if Two Strings Are Close
 */

// @lc code=start
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        let mut dict1 = HashMap::<char, usize>::new();
        let mut dict2 = HashMap::<char, usize>::new();
        for c in word1.chars() {
            dict1.entry(c).and_modify(|v| *v += 1).or_insert(1);
        }
        for c in word2.chars() {
            dict2.entry(c).and_modify(|v| *v += 1).or_insert(1);
        }
        let mut occur1 = dict1.values().collect::<Vec<_>>();
        occur1.sort();
        let mut occur2 = dict2.values().collect::<Vec<_>>();
        occur2.sort();

        dict1.keys().collect::<HashSet<_>>() == dict2.keys().collect::<HashSet<_>>()
            && occur1 == occur2
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
