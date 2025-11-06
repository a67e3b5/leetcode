/*
 * @lc app=leetcode id=2352 lang=rust
 *
 * [2352] Equal Row and Column Pairs
 */

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let mut n_pairs = 0;
        let n = grid.len();
        let mut occur: HashMap<Vec<i32>, i32> = HashMap::new();
        for row in &grid {
            *occur.entry(row.clone()).or_insert(0) += 1;
        }
        for c in 0..n {
            let mut col = Vec::with_capacity(n);
            for r in 0..n {
                col.push(grid[r][c]);
            }
            if let Some(&cnt) = occur.get(&col) {
                n_pairs += cnt;
            }
        }
        n_pairs
    }
}
// @lc code=end

struct Solution;
