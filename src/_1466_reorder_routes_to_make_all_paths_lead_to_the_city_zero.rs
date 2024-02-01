/*
 * @lc app=leetcode id=1466 lang=rust
 *
 * [1466] Reorder Routes to Make All Paths Lead to the City Zero
 */

// @lc code=start
impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let mut connections: Vec<Vec<usize>> = connections
            .into_iter()
            .map(|v| v.into_iter().map(|i| i as _).collect())
            .collect();
        let mut ans = 0;
        let mut has_access = vec![false; n as usize];
        has_access[0] = true;

        while has_access.iter().any(|&bool| !bool) {
            for c in &mut connections {
                let (c0, c1) = (c[0], c[1]);
                if !has_access[c1] && has_access[c0] {
                    c[1] = c0;
                    c[0] = c1;
                    ans += 1;
                    has_access[c1] = true;
                } else if !has_access[c0] && has_access[c1] {
                    has_access[c0] = true;
                }
            }
        }
        ans
    }
}
// @lc code=end

struct Solution;
