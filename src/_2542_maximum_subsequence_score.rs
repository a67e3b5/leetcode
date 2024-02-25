/*
 * @lc app=leetcode id=2542 lang=rust
 *
 * [2542] Maximum Subsequence Score
 */

// @lc code=start
use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
};
impl Solution {
    pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let mut zip = nums2
            .into_iter()
            .map(|n| n as i64)
            .zip(nums1.into_iter().map(|n| n as i64))
            .collect::<Vec<_>>();
        zip.sort_unstable();
        let mut ans = 0;
        let mut sum = 0;
        let mut heap = BinaryHeap::new();
        while let Some((n2, n1)) = zip.pop() {
            sum += n1;
            heap.push(Reverse(n1));
            match heap.len().cmp(&k) {
                Ordering::Less => continue,
                Ordering::Equal => (),
                Ordering::Greater => sum -= heap.pop().unwrap().0,
            }
            ans = ans.max(sum * n2)
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
