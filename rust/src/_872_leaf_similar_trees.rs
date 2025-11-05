/*
 * @lc app=leetcode id=872 lang=rust
 *
 * [872] Leaf-Similar Trees
 */

// @lc code=start
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        Self::leaves(&root1) == Self::leaves(&root2)
    }
    fn leaves(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let Some(node) = root else { return vec![] };
        let node_l = &node.borrow().left;
        let node_r = &node.borrow().right;
        if node_l.is_none() && node_r.is_none() {
            return vec![node.borrow().val];
        }
        [Self::leaves(node_l), Self::leaves(node_r)].concat()
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
