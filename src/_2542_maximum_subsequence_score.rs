/*
 * @lc app=leetcode id=2542 lang=rust
 *
 * [2542] Maximum Subsequence Score
 */

// @lc code=start
use std::collections::BinaryHeap;
impl Solution {
    pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let mut zip = nums1.into_iter().zip(nums2).collect::<Vec<_>>();
        zip.sort_unstable_by_key(|(_, n2)| *n2);
        let heap1 = zip
            .iter()
            .enumerate()
            .map(|(i, p)| (p.0, i))
            .collect::<BinaryHeap<_>>();
        let mut ans = 0;
        for (i, (n1, n2)) in zip.iter().enumerate().take(zip.len() - k + 1) {
            let (mut sum, mut count) = (*n1 as i64, 1);
            let mut heap1 = heap1.clone();
            while count < k {
                let (m1, j) = heap1.pop().unwrap();
                if i < j {
                    sum += m1 as i64;
                    count += 1;
                }
            }
            ans = ans.max(sum * *n2 as i64)
        }
        ans
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let samples = [((vec![1, 3, 3, 2], vec![2, 1, 3, 4], 3), 12)];
        for (input, output) in samples {
            assert_eq!(Solution::max_score(input.0, input.1, input.2), output);
        }
    }
}
