/*
 * @lc app=leetcode id=1448 lang=rust
 *
 * [1448] Count Good Nodes in Binary Tree
 */

// @lc code=start
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::_good_nodes(&root, i32::MIN)
    }
    fn _good_nodes(root: &Option<Rc<RefCell<TreeNode>>>, mut max_in_path: i32) -> i32 {
        let Some(node) = root else { return 0 };
        let this = if max_in_path <= node.borrow().val {
            max_in_path = node.borrow().val;
            1
        } else {
            0
        };
        this + Self::_good_nodes(&node.borrow().left, max_in_path)
            + Self::_good_nodes(&node.borrow().right, max_in_path)
    }
}
// @lc code=end

struct Solution;

// Definition for a binary tree node.
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
