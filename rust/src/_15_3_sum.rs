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
    /// https://leetcode.com/problems/3sum/solutions/5055810/video-two-pointer-solution-by-niits-cl7y/
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        nums.sort_unstable();

        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let mut j = i + 1;
            let mut k = nums.len() - 1;
            while j < k {
                let total = nums[i] + nums[j] + nums[k];
                if total > 0 {
                    k -= 1;
                } else if total < 0 {
                    j += 1;
                } else {
                    res.push(vec![nums[i], nums[j], nums[k]]);
                    j += 1;
                    while nums[j] == nums[j - 1] && j < k {
                        j += 1;
                    }
                }
            }
        }
        res
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
}
