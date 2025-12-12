/*
 * @lc app=leetcode id=909 lang=rust
 *
 * [909] Snakes and Ladders
 */

// @lc code=start
use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn snakes_and_ladders(mut board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();
        let mut cell_mut = |curr: usize| {
            let q = (curr - 1) / n;
            let r = (curr - 1) % n;
            let i = n - q - 1;
            let j = match q % 2 {
                0 => r,
                _ => n - r - 1,
            };
            let tmp = board[i][j];
            board[i][j] = 0;
            tmp
        };
        let mut queue = VecDeque::from([(1, 0)]);
        while let Some((curr, count)) = queue.pop_front() {
            let mut skip = false;
            for die in (1..=6).rev() {
                let next = curr + die;
                if n * n <= next {
                    return count + 1;
                }
                match cell_mut(next) {
                    -1 => {
                        if !skip {
                            queue.push_back((next, count + 1));
                            skip = true;
                        }
                    }
                    // already seen
                    0 => {}
                    x => {
                        if n * n <= x as usize {
                            return count + 1;
                        }
                        queue.push_back((x as usize, count + 1));
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
        let cases = [
            (
                vec![
                    vec![-1, -1, -1, -1, -1, -1],
                    vec![-1, -1, -1, -1, -1, -1],
                    vec![-1, -1, -1, -1, -1, -1],
                    vec![-1, 35, -1, -1, 13, -1],
                    vec![-1, -1, -1, -1, -1, -1],
                    vec![-1, 15, -1, -1, -1, -1],
                ],
                4,
            ),
            (vec![vec![-1, -1, -1], vec![-1, 9, 8], vec![-1, 8, 9]], 1),
            (
                vec![
                    vec![-1, 1, 2, -1],
                    vec![2, 13, 15, -1],
                    vec![-1, 10, -1, -1],
                    vec![-1, 6, 2, 8],
                ],
                2,
            ),
        ];
        for (args, ret) in cases {
            assert_eq!(
                super::Solution::snakes_and_ladders(args.clone()),
                ret,
                "case: {args:?}, {ret:?}"
            )
        }
    }
}
