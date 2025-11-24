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
        let neighbors = |(i, j): (usize, usize)| {
            let mut res = vec![];
            if i > 0 {
                res.push((i - 1, j));
            }
            if j > 0 {
                res.push((i, j - 1));
            }
            if i < m - 1 {
                res.push((i + 1, j));
            }
            if j < n - 1 {
                res.push((i, j + 1));
            }
            res
        };
        let mut ans = 0;
        let mut seen: HashSet<(usize, usize)> = HashSet::new();
        let Some(k) = grid.iter().flatten().position(|&c| c == '1') else {
            return ans;
        };
        let mut last_landing_index = k;
        let mut last_landing_point = (last_landing_index / n, last_landing_index % n);
        loop {
            ans += 1;
            let mut stack = vec![last_landing_point];
            while let Some((i, j)) = stack.pop() {
                seen.insert((i, j));
                let mut neighbors = neighbors((i, j));
                neighbors.retain(|p| on_land(p) && !seen.contains(p));
                neighbors.into_iter().for_each(|p| stack.push(p));
            }
            while seen.contains(&last_landing_point) {
                let offset = last_landing_index + 1;
                let Some(k) = grid.iter().flatten().skip(offset).position(|&c| c == '1') else {
                    return ans;
                };
                last_landing_index = offset + k;
                last_landing_point = (last_landing_index / n, last_landing_index % n);
            }
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
