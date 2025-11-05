/*
 * @lc app=leetcode id=199 lang=rust
 *
 * [199] Binary Tree Right Side View
 */

// @lc code=start
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let ans = vec![];
        let Some(rc) = root else { return ans };
        Self::_right_side_view(&[rc], ans)
    }
    fn _right_side_view(layer: &[Rc<RefCell<TreeNode>>], mut ans: Vec<i32>) -> Vec<i32> {
        let mut r_side = None;
        let mut children = vec![];
        for rc in layer {
            let node = rc.borrow();
            r_side = Some(node.val);
            if let Some(rc) = &node.left {
                children.push(rc.clone())
            }
            if let Some(rc) = &node.right {
                children.push(rc.clone())
            }
        }
        let Some(r_side) = r_side else { return ans };
        ans.push(r_side);
        Self::_right_side_view(&children, ans)
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
