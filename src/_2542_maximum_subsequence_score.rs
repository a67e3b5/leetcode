/*
 * @lc app=leetcode id=2542 lang=rust
 *
 * [2542] Maximum Subsequence Score
 */

// @lc code=start
impl Solution {
    pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let mut zip = nums1.into_iter().zip(nums2).collect::<Vec<_>>();
        zip.sort_unstable_by_key(|(_, n2)| *n2);
        let mut v1 = zip
            .iter()
            .enumerate()
            .map(|(i, p)| (p.0, i))
            .collect::<Vec<_>>();
        v1.sort_unstable_by_key(|(n1, _)| -*n1);
        let mut ans = 0;
        for (i, (n1, n2)) in zip.iter().enumerate().take(zip.len() - k + 1) {
            let sum = *n1 as i64
                + v1.iter()
                    .filter(|(_, j)| i < *j)
                    .take(k - 1)
                    .map(|(n, _)| *n as i64)
                    .sum::<i64>();
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
