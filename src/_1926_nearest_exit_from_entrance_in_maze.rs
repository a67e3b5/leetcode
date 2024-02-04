/*
 * @lc app=leetcode id=1926 lang=rust
 *
 * [1926] Nearest Exit from Entrance in Maze
 */

// @lc code=start
use std::collections::HashSet;
use std::collections::VecDeque;
impl Solution {
    pub fn nearest_exit(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        let m_rows = maze.len();
        let n_cols = maze[0].len();
        let is_empty = |c: char| c == '.';
        let mut front = VecDeque::new();
        let mut visited = HashSet::new();

        front.push_back((entrance[0] as usize, entrance[1] as usize, 0));
        while !front.is_empty() {
            while let Some((i, j, steps)) = front.pop_front() {
                if !visited.insert((i, j)) {
                    continue;
                }
                if (i == 0 || i == m_rows - 1 || j == 0 || j == n_cols - 1)
                    && (i != entrance[0] as usize || j != entrance[1] as usize)
                {
                    return steps;
                }
                if is_empty(maze[i][j]) {
                    if i > 0 && is_empty(maze[i - 1][j]) {
                        front.push_back((i - 1, j, steps + 1));
                    }
                    if i < m_rows - 1 && is_empty(maze[i + 1][j]) {
                        front.push_back((i + 1, j, steps + 1));
                    }
                    if j > 0 && is_empty(maze[i][j - 1]) {
                        front.push_back((i, j - 1, steps + 1));
                    }
                    if j < n_cols - 1 && is_empty(maze[i][j + 1]) {
                        front.push_back((i, j + 1, steps + 1));
                    }
                }
            }
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
            (
                (
                    vec![
                        vec!['+', '+', '+'],
                        vec!['.', '.', '.'],
                        vec!['+', '+', '+'],
                    ],
                    vec![1, 0],
                ),
                2,
            ),
            ((vec![vec!['.', '+']], vec![0, 0]), -1),
        ];
        for (input, output) in samples {
            assert_eq!(Solution::nearest_exit(input.0, input.1), output);
        }
    }
}
