/*
 * @lc app=leetcode id=1768 lang=rust
 *
 * [1768] Merge Strings Alternately
 */

// @lc code=start
impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let zip_len = std::cmp::min(word1.len(), word2.len());
        let (word1_zip, word1_rest) = word1.split_at(zip_len);
        let (word2_zip, word2_rest) = word2.split_at(zip_len);
        let mut res = String::new();
        for (c1, c2) in word1_zip.chars().zip(word2_zip.chars()) {
            res.push(c1);
            res.push(c2);
        }
        res.push_str(word1_rest);
        res.push_str(word2_rest);
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
