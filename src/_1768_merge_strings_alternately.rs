/*
 * @lc app=leetcode id=1768 lang=rust
 *
 * [1768] Merge Strings Alternately
 */

// @lc code=start
impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut word1_rev = word1.chars().rev().collect::<String>();
        let mut word2_rev = word2.chars().rev().collect::<String>();
        let mut res = String::new();
        loop {
            if let Some(c) = word1_rev.pop() {
                res.push(c)
            } else {
                if word2_rev.is_empty() {
                    break;
                }
            }
            if let Some(c) = word2_rev.pop() {
                res.push(c)
            } else {
                if word1_rev.is_empty() {
                    break;
                }
            }
        }
        res
    }
}
// @lc code=end

struct Solution;
