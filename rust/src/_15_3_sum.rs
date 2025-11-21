/*
 * @lc app=leetcode id=15 lang=rust
 *
 * [15] 3Sum
 */

// @lc code=start
use std::collections::HashMap;
use std::collections::HashSet;
use std::i32;

impl Solution {
    /// Reuse #1 logic.
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut nums_list: Vec<Vec<i32>> = Vec::new();
        let mut last_num = i32::MIN;
        for (i, &num) in nums.iter().enumerate().rev() {
            if num == last_num {
                continue;
            }
            last_num = num;
            for mut nums in Self::two_sum(&nums[..i], -num) {
                nums.push(num);
                nums_list.push(nums);
            }
        }
        nums_list.dedup();
        nums_list
    }

    /// Same as #1, except...
    fn two_sum(nums: &[i32], target: i32) -> Vec<Vec<i32>> {
        let mut seen: HashSet<i32> = HashSet::new();
        let mut nums_list = vec![];
        for &num in nums.iter() {
            let complement = target - num;
            if seen.contains(&complement) {
                nums_list.push([complement, num].into());
            } else {
                seen.insert(num);
            }
        }
        nums_list
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            super::Solution::three_sum(vec![-1, 0, 1, 2, -1, -4])
                .into_iter()
                .collect::<HashSet<_>>(),
            HashSet::from([vec![-1, 0, 1], vec![-1, -1, 2]])
        );
        assert_eq!(
            super::Solution::three_sum(vec![
                2, -3, 0, -2, -5, -5, -4, 1, 2, -2, 2, 0, 2, -4, 5, 5, -10
            ])
            .into_iter()
            .collect::<HashSet<_>>(),
            HashSet::from([
                vec![-10, 5, 5],
                vec![-5, 0, 5],
                vec![-4, 2, 2],
                vec![-3, -2, 5],
                vec![-3, 1, 2],
                vec![-2, 0, 2]
            ])
        );
        assert_eq!(
            super::Solution::three_sum(vec![0, 0, 0, 0]),
            vec![vec![0, 0, 0]]
        );
    }

    #[test]
    fn two() {
        assert_eq!(
            super::Solution::two_sum(&[-1, 0, 1, 2, -1, -4], 4),
            Vec::<Vec<_>>::new()
        );
        assert_eq!(
            super::Solution::two_sum(&[-1, 0, 1, 2, -1, -4], 1),
            vec![vec![0, 1], vec![-1, 2]]
        );
    }
}
