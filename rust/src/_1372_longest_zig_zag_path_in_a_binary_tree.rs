/*
 * @lc app=leetcode id=1372 lang=rust
 *
 * [1372] Longest ZigZag Path in a Binary Tree
 */

// @lc code=start
use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;
impl Solution {
    pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let Some(rc) = root else { return 0 };
        let node = rc.borrow();
        max(
            Self::_longest_zig_zag(&node.left, false, 0),
            Self::_longest_zig_zag(&node.right, true, 0),
        )
    }
    fn _longest_zig_zag(root: &Option<Rc<RefCell<TreeNode>>>, is_right: bool, acc_len: i32) -> i32 {
        let Some(rc) = root else { return acc_len };
        let node = rc.borrow();
        if is_right {
            max(
                Self::_longest_zig_zag(&node.left, false, acc_len + 1),
                Self::_longest_zig_zag(&node.right, true, 0),
            )
        } else {
            max(
                Self::_longest_zig_zag(&node.left, false, 0),
                Self::_longest_zig_zag(&node.right, true, acc_len + 1),
            )
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
