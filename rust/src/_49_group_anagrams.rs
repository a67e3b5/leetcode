/*
 * @lc app=leetcode id=49 lang=rust
 *
 * [49] Group Anagrams
 */

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<[usize; 26], Vec<String>> = HashMap::new();
        for s in strs {
            let mut freq = [0; 26];
            s.chars().for_each(|c| {
                let i = (c as u8 - b'a') as usize;
                freq[i] += 1;
            });
            map.entry(freq).or_default().push(s);
        }
        map.values().cloned().collect()
    }
}
// @lc code=end

struct Solution;
