/*
 * @lc app=leetcode id=424 lang=rust
 *
 * [424] Longest Repeating Character Replacement
 */

// @lc code=start
use std::{cmp::max, collections::HashMap};

#[derive(Debug)]
struct Value {
    since: usize,
    modify: i32,
    last_modified: usize,
    max_len: usize,
}

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let mut freq: HashMap<char, usize> = HashMap::new();
        let mut res = 0;
        let mut i = 0;
        for (j, &c) in s.iter().enumerate() {
            *freq.entry(c).or_default() += 1;
            let max_freq = freq.values().max().unwrap();
            let cur_len = j - i + 1;
            if cur_len - max_freq > k as usize {
                *freq.get_mut(&s[i]).unwrap() -= 1;
                i += 1;
            }
            res = max(res, j - i + 1)
        }
        res as i32
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let cases = [
            ("A", 0, 1),
            ("Z", 9, 1),
            ("BAAAB", 2, 5),
            ("BABABBA", 1, 4),
            ("AABABBA", 1, 4),
            (
                "IMNJJTRMJEGMSOLSCCQICIHLQIOGBJAEHQOCRAJQMBIBATGLJDTBNCPIFRDLRIJHRABBJGQAOLIKRLHDRIGERENNMJSDSSMESSTR",
                2,
                6,
            ),
        ];
        for (s, k, a) in cases {
            assert_eq!(
                super::Solution::character_replacement(s.to_string(), k),
                a,
                "s: {s}, k: {k}"
            );
        }
    }
}
