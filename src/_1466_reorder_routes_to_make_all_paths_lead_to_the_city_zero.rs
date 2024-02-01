/*
 * @lc app=leetcode id=1466 lang=rust
 *
 * [1466] Reorder Routes to Make All Paths Lead to the City Zero
 */

// @lc code=start
impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let mut graph: Vec<Vec<Edge>> = vec![vec![]; n as usize];
        for c in connections {
            graph[c[0] as usize].push(Edge::new(c[1] as usize, true));
            graph[c[1] as usize].push(Edge::new(c[0] as usize, false));
        }
        let mut nodes = vec![(n as usize, 0_usize)];

        while let Some((origin, node)) = nodes.pop() {
            for edge in &graph[node] {
                if edge.dest != origin {
                    if edge.is_reordered {
                        ans += 1;
                    }
                    nodes.push((node, edge.dest));
                }
            }
        }
        ans
    }
}

#[derive(Debug, Clone)]
struct Edge {
    dest: usize,
    is_reordered: bool,
}

impl Edge {
    #[inline]
    fn new(dest: usize, is_reordered: bool) -> Self {
        Self { dest, is_reordered }
    }
}
// @lc code=end

struct Solution;
