/*
 * @lc app=leetcode id=111 lang=rust
 *
 * [111] Minimum Depth of Binary Tree
 */

// @lc code=start
use std::cell::RefCell;
use std::cmp::min;
use std::rc::Rc;
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut stack = vec![(root, 1)];
        let mut min_depth = i32::MAX;
        while let Some((node, depth)) = stack.pop() {
            if let Some(node) = node {
                let l = node.borrow().left.clone();
                let r = node.borrow().right.clone();
                if l.is_some() || r.is_some() {
                    stack.push((l, depth + 1));
                    stack.push((r, depth + 1));
                } else {
                    min_depth = min(min_depth, depth);
                }
            }
        }
        if min_depth < i32::MAX { min_depth } else { 0 }
    }
}
// @lc code=end
struct Solution;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
