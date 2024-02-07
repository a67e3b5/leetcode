/*
 * @lc app=leetcode id=1143 lang=rust
 *
 * [1143] Longest Common Subsequence
 */

// @lc code=start
impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let m = text1.len();
        let n = text2.len();
        let mut match_grid = vec![vec![false; n]; m];
        for (i, c1) in text1.char_indices() {
            for (j, c2) in text2.char_indices() {
                if c1 == c2 {
                    match_grid[i][j] = true;
                }
            }
        }
        let mut count_grid = vec![vec![0; n]; m];
        for i in 0..m {
            for j in 0..n {
                let inc = if match_grid[i][j] {
                    (i..m).for_each(|i| match_grid[i][j] = false);
                    (j..n).for_each(|j| match_grid[i][j] = false);
                    1
                } else {
                    0
                };
                let u = if i == 0 { 0 } else { count_grid[i - 1][j] };
                let l = if j == 0 { 0 } else { count_grid[i][j - 1] };
                count_grid[i][j] = inc + u.max(l);
            }
        }
        count_grid[m - 1][n - 1]
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        const LCSS: &str = "longestcommonsubsequence";
        const XXXX: &str = "xxxxxxxxxxxxxxxxxxxxxxxx";
        let samples = [
            (
                (&*format!("x{}{}", XXXX, LCSS), &*format!("{}x", LCSS)),
                LCSS.len(),
            ),
            (("bsbininm", "jmjkbkjkv"), 1),
            (("pmjghexybyrgzczy", "hafcdqbgncrcbihkd"), 4),
        ];
        for (input, output) in samples {
            assert_eq!(
                Solution::longest_common_subsequence(input.0.to_string(), input.1.to_string()),
                output as i32
            );
        }
    }
}
