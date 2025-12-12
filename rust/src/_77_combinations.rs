/*
 * @lc app=leetcode id=77 lang=rust
 *
 * [77] Combinations
 */

// @lc code=start
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        Self::backtrack(n, k as usize, 1, &mut vec![], &mut result);
        result
    }

    fn backtrack(n: i32, k: usize, pos: i32, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if current.len() == k {
            result.push(current.clone());
            return;
        }
        for choice in pos..=n {
            current.push(choice);
            Self::backtrack(n, k, choice + 1, current, result);
            current.pop();
        }
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let cases = [(
            4,
            2,
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![1, 4],
                vec![2, 3],
                vec![2, 4],
                vec![3, 4],
            ],
        )];
        for (n, k, ret) in cases {
            assert_eq!(
                super::Solution::combine(n, k),
                ret,
                "case: {n}, {k}, {ret:?}"
            )
        }
    }
}
