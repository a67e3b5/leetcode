/*
 * @lc app=leetcode id=112 lang=rust
 *
 * [112] Path Sum
 */

// @lc code=start
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        let mut stack = vec![(root, target_sum)];
        while let Some((node, sum)) = stack.pop() {
            if let Some(node) = node {
                let v = node.borrow().val;
                let l = node.borrow().left.clone();
                let r = node.borrow().right.clone();
                if l.is_some() || r.is_some() {
                    stack.push((l, sum - v));
                    stack.push((r, sum - v));
                } else if sum - v == 0 {
                    return true;
                }
            }
        }
        false
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
