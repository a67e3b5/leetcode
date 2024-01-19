/*
 * @lc app=leetcode id=724 lang=rust
 *
 * [724] Find Pivot Index
 */

// @lc code=start
impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let sum = nums.iter().sum::<i32>();
        let lr_sums = nums.iter().map(|n| sum - n);
        let l_sums = std::iter::once(&0).chain(nums.iter()).scan(0, |st, x| {
            *st += *x;
            Some(*st)
        });
        if let Some(pivot_i) = lr_sums
            .zip(l_sums)
            .enumerate()
            .find_map(|(i, (lr, l))| (lr == l * 2).then_some(i))
        {
            return pivot_i as i32;
        }
        -1
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let samples = [
            (vec![1], 0),
            (vec![1, 1], -1),
            (vec![0, 1, -1, 0], 0),
            (vec![1, -1, 0], 2),
        ];
        for (src, res) in samples {
            assert_eq!(Solution::pivot_index(src), res)
        }
    }
}
