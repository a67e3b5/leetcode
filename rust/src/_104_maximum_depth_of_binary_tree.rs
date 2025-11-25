/*
 * @lc app=leetcode id=104 lang=rust
 *
 * [104] Maximum Depth of Binary Tree
 */

// @lc code=start
use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut stack = vec![(root, 1)];
        let mut max_depth = 0;
        while let Some((node, depth)) = stack.pop() {
            if let Some(node) = node {
                max_depth = max(max_depth, depth);
                stack.push((node.borrow().left.clone(), depth + 1));
                stack.push((node.borrow().right.clone(), depth + 1));
            }
        }
        max_depth
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
