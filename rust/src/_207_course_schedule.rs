/*
 * @lc app=leetcode id=207 lang=rust
 *
 * [207] Course Schedule
 */

use std::{cmp::max, collections::HashSet, vec};

// @lc code=start
impl Solution {
    pub fn can_finish(num_courses: i32, mut prerequisites: Vec<Vec<i32>>) -> bool {
        let mut visited = HashSet::new();
        // search upward
        let mut upper_depth = 0;
        prerequisites.sort_unstable_by_key(|p| p[1]);
        let mut stack = vec![(prerequisites[0][0], 0)];
        visited.insert(prerequisites[0][0]);
        while let Some((c, depth)) = stack.pop() {
            upper_depth = max(upper_depth, depth);
            let Ok(offset) = prerequisites.binary_search_by_key(&c, |p| p[1]) else {
                continue;
            };
            let next = prerequisites
                .iter()
                .skip(offset)
                .take_while(|p| p[1] == c)
                .filter(|p| !visited.contains(&p[0]))
                .map(|p| (p[0], depth + 1))
                .collect::<Vec<_>>();
            next.iter().for_each(|(c, _depth)| {
                visited.insert(*c);
            });
            stack.extend(next);
        }
        // search downward
        let mut downer_depth = 0;
        prerequisites.sort_unstable_by_key(|p| p[0]);
        let mut stack = vec![(prerequisites[0][1], 0)];
        visited.insert(prerequisites[0][1]);
        while let Some((c, depth)) = stack.pop() {
            downer_depth = max(downer_depth, depth);
            let Ok(offset) = prerequisites.binary_search_by_key(&c, |p| p[0]) else {
                continue;
            };
            let next = prerequisites
                .iter()
                .skip(offset)
                .take_while(|p| p[0] == c)
                .filter(|p| !visited.contains(&p[1]))
                .map(|p| (p[1], depth + 1))
                .collect::<Vec<_>>();
            next.iter().for_each(|(c, _depth)| {
                visited.insert(*c);
            });
            stack.extend(next);
        }
        upper_depth + downer_depth >= num_courses
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
