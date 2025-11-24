/*
 * @lc app=leetcode id=994 lang=rust
 *
 * [994] Rotting Oranges
 */

// @lc code=start
use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len() as isize;
        let n = grid[0].len() as isize;
        let mut rotten = VecDeque::new();
        for i in 0..m {
            for j in 0..n {
                if grid[i as usize][j as usize] == 2 {
                    rotten.push_back((i, j, 0));
                }
            }
        }
        let mut bfs = |mut queue: VecDeque<(isize, isize, i32)>| -> i32 {
            let directions: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            let mut elapsed = 0;
            while let Some((i, j, t)) = queue.pop_front() {
                for (di, dj) in directions {
                    let i = i + di;
                    let j = j + dj;
                    if 0 <= i && i < m && 0 <= j && j < n && grid[i as usize][j as usize] == 1 {
                        queue.push_back((i, j, t + 1));
                        grid[i as usize][j as usize] = 2;
                    }
                }
                elapsed = t;
            }
            elapsed
        };
        let elapsed = bfs(rotten);
        if grid.iter().flatten().any(|&o| o == 1) {
            -1
        } else {
            elapsed
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
            (vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]], 4),
            (vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]], -1),
            (vec![vec![0, 2]], 0),
        ];
        for (args, ret) in cases {
            assert_eq!(
                super::Solution::oranges_rotting(args.clone()),
                ret,
                "case: {args:?}, {ret}"
            )
        }
    }
}
