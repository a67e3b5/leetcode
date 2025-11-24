/*
 * @lc app=leetcode id=200 lang=rust
 *
 * [200] Number of Islands
 */

// @lc code=start
use std::collections::HashSet;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let on_land = |&(i, j): &(usize, usize)| grid[i][j] == '1';
        let dfs = |(i, j): (usize, usize), seen: &mut HashSet<(usize, usize)>| {
            let mut stack = vec![(i, j)];
            seen.insert((i, j));

            while let Some((i, j)) = stack.pop() {
                if i > 0 && on_land(&(i - 1, j)) && !seen.contains(&(i - 1, j)) {
                    stack.push((i - 1, j));
                    seen.insert((i - 1, j));
                }
                if j > 0 && on_land(&(i, j - 1)) && !seen.contains(&(i, j - 1)) {
                    stack.push((i, j - 1));
                    seen.insert((i, j - 1));
                }
                if i < m - 1 && on_land(&(i + 1, j)) && !seen.contains(&(i + 1, j)) {
                    stack.push((i + 1, j));
                    seen.insert((i + 1, j));
                }
                if j < n - 1 && on_land(&(i, j + 1)) && !seen.contains(&(i, j + 1)) {
                    stack.push((i, j + 1));
                    seen.insert((i, j + 1));
                }
            }
        };
        let mut ans = 0;
        let mut seen: HashSet<(usize, usize)> = HashSet::new();
        for i in 0..m {
            for j in 0..n {
                if on_land(&(i, j)) && !seen.contains(&(i, j)) {
                    ans += 1;
                    dfs((i, j), &mut seen);
                }
            }
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
        let cases = [
            (
                vec![
                    vec!['1', '1', '1', '1', '0'],
                    vec!['1', '1', '0', '1', '0'],
                    vec!['1', '1', '0', '0', '0'],
                    vec!['0', '0', '0', '0', '0'],
                ],
                1,
            ),
            (
                vec![
                    vec!['1', '1', '0', '0', '0'],
                    vec!['1', '1', '0', '0', '0'],
                    vec!['0', '0', '1', '0', '0'],
                    vec!['0', '0', '0', '1', '1'],
                ],
                3,
            ),
            (
                vec![
                    vec!['1', '1', '1'],
                    vec!['0', '1', '0'],
                    vec!['1', '1', '1'],
                ],
                1,
            ),
        ];
        for (args, ret) in cases {
            assert_eq!(
                super::Solution::num_islands(args.clone()),
                ret,
                "case: {args:?}, {ret}"
            )
        }
    }
}
