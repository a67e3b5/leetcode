/*
 * @lc app=leetcode id=54 lang=rust
 *
 * [54] Spiral Matrix
 */

// @lc code=start
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let m = matrix.len();
        let n = matrix[0].len();
        if n == 1 {
            return matrix.concat();
        }
        let mut cursor = Cursor {
            i: 0,
            j: 0,
            top_end: 0,
            bottom_end: m - 1,
            left_end: 0,
            right_end: n - 1,
            direction: Direction::Up,
        };
        let mut count = m * n;
        let mut ans = Vec::new();
        while count > 0 {
            ans.push(matrix[cursor.i][cursor.j]);
            count -= 1;
            if count == 0 {
                break;
            }
            while cursor.is_corner() {
                cursor.change_direction();
            }
            cursor.shift();
        }
        ans
    }
}

struct Cursor {
    i: usize,
    j: usize,
    top_end: usize,
    bottom_end: usize,
    left_end: usize,
    right_end: usize,
    direction: Direction,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Cursor {
    fn is_corner(&self) -> bool {
        match self.direction {
            Direction::Up => self.top_end == self.i,
            Direction::Down => self.bottom_end == self.i,
            Direction::Left => self.left_end == self.j,
            Direction::Right => self.right_end == self.j,
        }
    }

    fn shift(&mut self) {
        match self.direction {
            Direction::Up => self.i -= 1,
            Direction::Down => self.i += 1,
            Direction::Left => self.j -= 1,
            Direction::Right => self.j += 1,
        }
    }

    fn change_direction(&mut self) {
        match self.direction {
            Direction::Up => {
                self.direction = Direction::Right;
                self.top_end += 1;
            }
            Direction::Down => {
                self.direction = Direction::Left;
                self.bottom_end -= 1;
            }
            Direction::Left => {
                self.direction = Direction::Up;
                self.left_end += 1;
            }
            Direction::Right => {
                self.direction = Direction::Down;
                self.right_end -= 1;
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
                vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
                vec![1, 2, 3, 6, 9, 8, 7, 4, 5],
            ),
            (
                vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]],
                vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7],
            ),
            (vec![vec![1]], vec![1]),
            (vec![vec![3, 2]], vec![3, 2]),
            (vec![vec![3],vec![2]], vec![3, 2]),
        ];
        for (args, ret) in cases {
            assert_eq!(
                super::Solution::spiral_order(args.clone()),
                ret,
                "case: {args:?}, {ret:?}"
            )
        }
    }
}
