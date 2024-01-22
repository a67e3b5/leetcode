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
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::_search_bst(&root, val)
    }
    fn _search_bst(
        root: &Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let node = root.as_ref()?;
        match node.borrow().val.cmp(&val) {
            Ordering::Equal => root.to_owned(),
            Ordering::Greater => Self::_search_bst(&node.borrow().left, val),
            Ordering::Less => Self::_search_bst(&node.borrow().right, val),
        }
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
