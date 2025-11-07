/*
 * @lc app=leetcode id=649 lang=rust
 *
 * [649] Dota2 Senate
 */

// @lc code=start
use std::collections::VecDeque;

impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let n = senate.len();
        let mut r = VecDeque::new();
        let mut d = VecDeque::new();

        for (i, ch) in senate.chars().enumerate() {
            if ch == 'R' {
                r.push_back(i);
            } else {
                d.push_back(i);
            }
        }

        while !r.is_empty() && !d.is_empty() {
            let ri = r.pop_front().unwrap();
            let di = d.pop_front().unwrap();
            if ri < di {
                r.push_back(ri + n);
            } else {
                d.push_back(di + n);
            }
        }

        if r.is_empty() {
            "Dire".to_string()
        } else {
            "Radiant".to_string()
        }
    }
}

// @lc code=end

struct Solution;
