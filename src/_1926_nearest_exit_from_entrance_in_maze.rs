/*
 * @lc app=leetcode id=1926 lang=rust
 *
 * [1926] Nearest Exit from Entrance in Maze
 */

// @lc code=start
impl Solution {
    pub fn nearest_exit(mut maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        const WALL: char = '+';
        let entrance = (entrance[0] as usize, entrance[1] as usize);
        let m_rows = maze.len();
        let n_cols = maze[0].len();
        let is_wall = |c: char| c == WALL;
        let on_edge =
            |(i, j): (usize, usize)| i == 0 || i == m_rows - 1 || j == 0 || j == n_cols - 1;
        let mut steps = 0;
        let mut front = vec![entrance];

        if on_edge(entrance) {
            let (i, j) = entrance;
            front.pop();
            if i != 0 && !is_wall(maze[i - 1][j]) {
                front.push((i - 1, j));
            }
            if i != m_rows - 1 && !is_wall(maze[i + 1][j]) {
                front.push((i + 1, j));
            }
            if j != 0 && !is_wall(maze[i][j - 1]) {
                front.push((i, j - 1));
            }
            if j != n_cols - 1 && !is_wall(maze[i][j + 1]) {
                front.push((i, j + 1));
            }
            maze[i][j] = WALL;
            steps += 1;
        }

        while !front.is_empty() {
            let mut next = vec![];
            for (i, j) in front {
                if on_edge((i, j)) {
                    return steps;
                }
                if is_wall(maze[i][j]) {
                    continue;
                }
                if !is_wall(maze[i - 1][j]) {
                    next.push((i - 1, j))
                }
                if !is_wall(maze[i + 1][j]) {
                    next.push((i + 1, j))
                }
                if !is_wall(maze[i][j - 1]) {
                    next.push((i, j - 1))
                }
                if !is_wall(maze[i][j + 1]) {
                    next.push((i, j + 1))
                }
                // Bury with a wall the current location so as not to be revisited.
                maze[i][j] = WALL;
            }
            front = next;
            steps += 1;
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
