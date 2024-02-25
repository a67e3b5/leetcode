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
        let mut zip = nums2.into_iter().zip(nums1).collect::<BinaryHeap<_>>();
        let mut ans = 0;
        let mut heap = BinaryHeap::new();
        (0..k - 1).for_each(|_| heap.push(zip.pop().unwrap().1));
        while let Some((n2, n1)) = zip.pop() {
            let mut h1 = heap.clone();
            let sum = (0..k - 1).fold(n1 as i64, |acc, _| acc + h1.pop().unwrap() as i64);
            heap.push(n1);
            ans = ans.max(sum * n2 as i64)
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
