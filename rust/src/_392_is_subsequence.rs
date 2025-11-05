/*
 * @lc app=leetcode id=392 lang=rust
 *
 * [392] Is Subsequence
 */

// @lc code=start
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let s = s.chars().collect::<Vec<_>>();
        let t = t.chars().collect::<Vec<_>>();
        let len_s = s.len();
        let len_t = t.len();
        let mut idx_s = 0;
        let mut idx_t = 0;
        while idx_s < len_s && idx_t < len_t {
            if s[idx_s] == t[idx_t] {
                idx_s += 1;
            }
            idx_t += 1;
        }
        idx_s == len_s
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let samples = [("abc", "ahbgdc", true), ("axc", "ahbgdc", false)];
        for (sub, sup, res) in samples {
            assert_eq!(
                Solution::is_subsequence(sub.to_string(), sup.to_string()),
                res
            );
        }
    }
}
