/*
 * @lc app=leetcode id=347 lang=rust
 *
 * [347] Top K Frequent Elements
 */

// @lc code=start
impl Solution {
    pub fn top_k_frequent(mut nums: Vec<i32>, k: i32) -> Vec<i32> {
        nums.sort_unstable();
        let mut freqs: Vec<(usize, i32)> = nums
            .chunk_by(|a, b| a == b)
            .map(|chunk| (chunk.len(), chunk[0]))
            .collect();
        freqs.sort_unstable();
        freqs[freqs.len() - k as usize..]
            .iter()
            .map(|(_len, num)| *num)
            .collect()
    }
}
// @lc code=end

struct Solution;
