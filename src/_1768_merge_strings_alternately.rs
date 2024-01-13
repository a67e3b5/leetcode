/*
 * @lc app=leetcode id=1768 lang=rust
 *
 * [1768] Merge Strings Alternately
 */

// @lc code=start
impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let [word1_len, word2_len] = [&word1, &word2].map(|w| w.len());
        let mut chars1 = word1.chars();
        let mut chars2 = word2.chars();
        let mut res = String::new();
        for i in 0..std::cmp::max(word1_len, word2_len) {
            if i < word1_len {
                res.push(chars1.next().unwrap())
            }
            if i < word2_len {
                res.push(chars2.next().unwrap())
            }
        }
        res
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let word1 = "qwe".to_string();
        let word2 = "asdfg".to_string();
        let result = Solution::merge_alternately(word1, word2);
        let expected = "qawsedfg".to_string();
        assert_eq!(result, expected);
    }
}
