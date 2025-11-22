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
        let mut map: HashMap<char, Value> = ('A'..='Z')
            .map(|c| {
                (
                    c,
                    Value {
                        since: 0,
                        modify: k,
                        last_modified: 0,
                        max_len: 0,
                    },
                )
            })
            .collect();
        for (&c, v) in map.iter_mut() {
            let mut r = 0;
            while r < s.len() {
                if s[r] == c {
                    r += 1;
                } else if v.modify > 0 {
                    v.modify -= 1;
                    v.last_modified = r;
                    r += 1;
                } else if k > 0 {
                    v.max_len = max(v.max_len, r - v.since);
                    v.since = v.last_modified + 1;
                    v.modify = k;
                } else {
                    v.max_len = max(v.max_len, r - v.since);
                    r += 1;
                    v.since = r;
                }
            }
            v.max_len = max(v.max_len, r - v.since)
        }
        map.values().map(|v| v.max_len).max().unwrap() as i32
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
            ("IMNJJTRMJEGMSOLSCCQICIHLQIOGBJAEHQOCRAJQMBIBATGLJDTBNCPIFRDLRIJHRABBJGQAOLIKRLHDRIGERENNMJSDSSMESSTR", 2, 6),
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
