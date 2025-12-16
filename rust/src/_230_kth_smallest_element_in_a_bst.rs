/*
 * @lc app=leetcode id=230 lang=rust
 *
 * [230] Kth Smallest Element in a BST
 */

// @lc code=start

use core::borrow;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn kth_smallest(mut root: Option<Rc<RefCell<TreeNode>>>, mut k: i32) -> i32 {
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        while root.is_some() || !stack.is_empty() {
            while let Some(node) = root {
                root = node.borrow().left.clone();
                stack.push(node);
            }
            let node = stack.pop().unwrap();
            k -= 1;
            if k == 0 {
                return node.borrow().val;
            }
            root = node.borrow().right.clone();
        }
        unreachable!()
    }
}
// @lc code=end

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

struct Solution;
