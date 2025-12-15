/*
 * @lc app=leetcode id=120 lang=rust
 *
 * [120] Triangle
 */

// @lc code=start
impl Solution {
    pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
        let m = triangle.len();
        for i in 1..m {
            let n = triangle[i].len();
            for j in 0..n {
                if 0 < j && j < n - 1 {
                    triangle[i][j] += triangle[i - 1][j - 1].min(triangle[i - 1][j])
                } else if 0 == j {
                    triangle[i][j] += triangle[i - 1][j]
                } else {
                    triangle[i][j] += triangle[i - 1][j - 1]
                }
            }
        }
        *triangle[m - 1].iter().min().unwrap()
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
            (
                vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]],
                11,
            ),
            (vec![vec![-10]], -10),
        ];
        for (args, ret) in cases {
            assert_eq!(
                super::Solution::minimum_total(args.clone()),
                ret,
                "case: {args:?}, {ret:?}"
            )
        }
    }
}
