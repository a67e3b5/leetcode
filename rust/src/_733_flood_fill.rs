/*
 * @lc app=leetcode id=733 lang=rust
 *
 * [733] Flood Fill
 */

// @lc code=start
impl Solution {
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let m = image.len() as i32;
        let n = image[0].len() as i32;
        let origin = (sr, sc);
        let origin_color = image[sr as usize][sc as usize];
        if color == origin_color {
            return image;
        }
        // DFS to mutate image
        let mut stack = vec![origin];
        image[sr as usize][sc as usize] = color;
        let directions: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        while let Some(p) = stack.pop() {
            for (di, dj) in directions {
                let i = p.0 + di;
                let j = p.1 + dj;
                if 0 <= i
                    && i < m
                    && 0 <= j
                    && j < n
                    && image[i as usize][j as usize] == origin_color
                {
                    stack.push((i, j));
                    image[i as usize][j as usize] = color;
                }
            }
        }
        image
    }
}
// @lc code=end

struct Solution;
