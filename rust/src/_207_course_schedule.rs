/*
 * @lc app=leetcode id=207 lang=rust
 *
 * [207] Course Schedule
 */

// @lc code=start

use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    /// Topological sort by Kahn algorithm.
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut graph: HashMap<usize, HashSet<usize>> = HashMap::new();
        let mut indegree = vec![0; num_courses as usize];
        prerequisites.into_iter().for_each(|p| {
            graph
                .entry(p[1] as usize)
                .or_default()
                .insert(p[0] as usize);
            indegree[p[0] as usize] += 1;
        });
        let mut queue = VecDeque::new();
        indegree
            .iter()
            .enumerate()
            .filter(|(_c, ind)| **ind == 0)
            .for_each(|(c, _ind)| queue.push_back(c));
        let mut sorted = vec![];
        while let Some(p) = queue.pop_front() {
            sorted.push(p);
            if let Some(cs) = graph.get(&p) {
                for c in cs {
                    indegree[*c] -= 1;
                    if indegree[*c] == 0 {
                        queue.push_back(*c);
                    }
                }
            }
        }
        sorted.len() as i32 == num_courses
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
            (2, vec![vec![1, 0]], true),
            (2, vec![vec![1, 0], vec![0, 1]], false),
        ];
        for (num, req, ret) in cases {
            assert_eq!(
                super::Solution::can_finish(num, req.clone()),
                ret,
                "case: {num}, {req:?}, {ret}"
            )
        }
    }
}
