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
        let mut ans = 0;
        let n = zip.len();
        for (i, (n1, n2)) in zip.iter().enumerate().take(n - k + 1) {
            let mut v1 = zip
                .iter()
                .skip(i + 1)
                .map(|(n1, _)| *n1 as i64)
                .collect::<Vec<_>>();
            v1.sort_unstable();
            ans = ans.max((*n1 as i64 + v1.iter().skip(n - k - i).sum::<i64>()) * *n2 as i64)
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
