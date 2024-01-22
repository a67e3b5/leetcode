/*
 * @lc app=leetcode id=700 lang=rust
 *
 * [700] Search in a Binary Search Tree
 */

// @lc code=start
use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;
impl Solution {
    pub fn search_bst(
        mut root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        while let Some(node) = root {
            match node.borrow().val.cmp(&val) {
                Ordering::Equal => return Some(node.clone()),
                Ordering::Greater => root = node.borrow().left.to_owned(),
                Ordering::Less => root = node.borrow().right.to_owned(),
            }
        }
        None
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
