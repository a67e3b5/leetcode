/*
 * @lc app=leetcode id=207 lang=rust
 *
 * [207] Course Schedule
 */

use std::{cmp::max, collections::{HashMap, HashSet}, mem::take, vec};

// @lc code=start
impl Solution {
    /// Check if asny course has a loop in the dependency graph.
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut graph: HashMap<i32, HashSet<i32>> = HashMap::new();
        prerequisites.into_iter()
            .for_each(|p| {
                graph.entry(p[0]).or_default().insert(p[1]);
            });
        // universal over courses?
        let mut taken = HashSet::new();
        let mut dfs = |course: i32| -> bool {
            let mut stack = vec![course];
            taken.insert(course);
            while let Some(c) = stack.pop() {
                let Some(ps) = graph.remove(&c) else {
                    continue;
                };
                for p in ps {
                    if taken.contains(&p) {
                        return false;
                    }
                    stack.push(p);
                    taken.insert(p);
                }
            }
            true
        };
        (0..num_courses).all(|c| dfs(c))
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
