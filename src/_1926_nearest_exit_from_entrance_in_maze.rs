/*
 * @lc app=leetcode id=1926 lang=rust
 *
 * [1926] Nearest Exit from Entrance in Maze
 */

// @lc code=start
impl Solution {
    pub fn nearest_exit(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        let mut maze: Vec<Vec<bool>> = maze
            .into_iter()
            .map(|v| v.into_iter().map(|c| c == '.').collect())
            .collect();
        let entrance = (entrance[0] as usize, entrance[1] as usize);
        let m_rows = maze.len();
        let n_cols = maze[0].len();
        let is_exit =
            |p: (usize, usize)| p.0 == 0 || p.0 == m_rows - 1 || p.1 == 0 || p.1 == n_cols - 1;
        let mut steps = 0;
        let mut front = vec![entrance];

        if is_exit(entrance) {
            let (row, col) = entrance;
            front.pop();
            if row != 0 && maze[row - 1][col] {
                front.push((row - 1, col));
            }
            if row != m_rows - 1 && maze[row + 1][col] {
                front.push((row + 1, col));
            }
            if col != 0 && maze[row][col - 1] {
                front.push((row, col - 1));
            }
            if col != n_cols - 1 && maze[row][col + 1] {
                front.push((row, col + 1));
            }
            maze[row][col] = false;
            steps += 1;
        }

        loop {
            let mut next = vec![];
            for (row, col) in front {
                if is_exit((row, col)) {
                    return steps;
                }
                if maze[row - 1][col] {
                    next.push((row - 1, col));
                }
                if maze[row + 1][col] {
                    next.push((row + 1, col));
                }
                if maze[row][col - 1] {
                    next.push((row, col - 1));
                }
                if maze[row][col + 1] {
                    next.push((row, col + 1));
                }
                // Bury with a wall the current location so as not to be revisited.
                maze[row][col] = false;
            }
            if next.is_empty() {
                return -1;
            }
            front = next;
            steps += 1;
        }
    }
}
// @lc code=end

struct Solution;
