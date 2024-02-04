/*
 * @lc app=leetcode id=1926 lang=rust
 *
 * [1926] Nearest Exit from Entrance in Maze
 */

// @lc code=start
impl Solution {
    pub fn nearest_exit(mut maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        const WALL: char = '+';
        const FRONT: char = '?';
        let entrance = (entrance[0] as usize, entrance[1] as usize);
        let m_rows = maze.len();
        let n_cols = maze[0].len();
        let is_wall = |c: char| c == WALL;
        let is_front = |c: char| c == FRONT;
        let is_exit =
            |p: (usize, usize)| p.0 == 0 || p.0 == m_rows - 1 || p.1 == 0 || p.1 == n_cols - 1;
        let mut steps = 0;

        maze[entrance.0][entrance.1] = FRONT;

        if is_exit(entrance) {
            let (row, col) = entrance;
            if row != 0 && !is_wall(maze[row - 1][col]) {
                maze[row - 1][col] = FRONT;
            }
            if row != m_rows - 1 && !is_wall(maze[row + 1][col]) {
                maze[row + 1][col] = FRONT;
            }
            if col != 0 && !is_wall(maze[row][col - 1]) {
                maze[row][col - 1] = FRONT;
            }
            if col != n_cols - 1 && !is_wall(maze[row][col + 1]) {
                maze[row][col + 1] = FRONT;
            }
            maze[row][col] = WALL;
            steps += 1;
        }

        loop {
            let mut next = vec![];
            for row in 0..m_rows {
                for col in 0..n_cols {
                    if !is_front(maze[row][col]) {
                        continue;
                    }
                    if is_exit((row, col)) {
                        return steps;
                    }
                    if !is_wall(maze[row - 1][col]) {
                        next.push((row - 1, col))
                    }
                    if !is_wall(maze[row + 1][col]) {
                        next.push((row + 1, col))
                    }
                    if !is_wall(maze[row][col - 1]) {
                        next.push((row, col - 1))
                    }
                    if !is_wall(maze[row][col + 1]) {
                        next.push((row, col + 1))
                    }
                    // Bury with a wall the current location so as not to be revisited.
                    maze[row][col] = WALL;
                }
            }
            if next.is_empty() {
                return -1;
            }
            for (row, col) in next {
                maze[row][col] = FRONT;
            }
            steps += 1;
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
