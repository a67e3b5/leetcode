/*
 * @lc app=leetcode id=135 lang=rust
 *
 * [135] Candy
 */

// @lc code=start
use std::{cmp::min, collections::HashMap};

impl Solution {
    pub fn candy(mut ratings: Vec<i32>) -> i32 {
        let n = ratings.len();
        let mut last = ratings[0];
        ratings[0] = 0i32;
        let mut bottom = ratings[0];
        for i in 1..n {
            let tmp = ratings[i];
            if last < tmp {
                ratings[i] = ratings[i - 1] + 1;
            } else {
                ratings[i] = ratings[i - 1] - 1;
                bottom = min(bottom, ratings[i]);
            }
            last = tmp;
        }
        n as i32 * (1 - bottom) + ratings.iter().sum::<i32>()
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let cases = [
            (vec![0], 1),
            (vec![1], 1),
            // FIXME
            (vec![1, 1], 2),
            (vec![1, 0, 2], 5),
            (vec![1, 2, 2], 4),
        ];
        for (args, ret) in cases {
            assert_eq!(
                super::Solution::candy(args.clone()),
                ret,
                "case: {args:?}, {ret:?}"
            )
        }
    }
}
