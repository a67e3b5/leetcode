/*
 * @lc app=leetcode id=15 lang=rust
 *
 * [15] 3Sum
 */

// @lc code=start
use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    /// Reuse #1 logic.
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums_set: HashSet<Vec<i32>> = HashSet::new();
        for (i, &num) in nums.iter().enumerate().rev() {
            for mut indices in Self::two_sum(&nums, -num) {
                if i <= *indices.last().unwrap() {
                    continue;
                }
                indices.push(i);
                let mut nums: Vec<i32> = indices.into_iter().map(|i| nums[i]).collect();
                nums.sort_unstable();
                nums_set.insert(nums);
            }
        }
        dbg!(&nums_set);
        nums_set.into_iter().collect()
    }

    /// Same as #1, except...
    fn two_sum(nums: &[i32], target: i32) -> Vec<Vec<usize>> {
        let mut seen: HashMap<i32, usize> = HashMap::new();
        let mut indices_list = vec![];
        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(&j) = seen.get(&complement) {
                indices_list.push([j, i].into());
            } else {
                seen.insert(num, i);
            }
        }
        indices_list
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
    }

    #[test]
    fn two() {
        assert_eq!(
            super::Solution::two_sum(&[-1, 0, 1, 2, -1, -4], 4),
            Vec::<Vec<_>>::new()
        );
        assert_eq!(
            super::Solution::two_sum(&[-1, 0, 1, 2, -1, -4], 1),
            vec![vec![1, 2], vec![0, 3]]
        );
    }
}
