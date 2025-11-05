/*
 * @lc app=leetcode id=104 lang=rust
 *
 * [104] Maximum Depth of Binary Tree
 */

// @lc code=start
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::_max_depth(&root)
    }
    fn _max_depth(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            return 1 + std::cmp::max(
                Self::_max_depth(&node.borrow().left),
                Self::_max_depth(&node.borrow().right),
            );
        }
        0
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
