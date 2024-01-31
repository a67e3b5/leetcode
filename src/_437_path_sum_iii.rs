/*
 * @lc app=leetcode id=437 lang=rust
 *
 * [437] Path Sum III
 */

// @lc code=start
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        Self::_path_sum(&root, target_sum, &[])
    }
    fn _path_sum(root: &Option<Rc<RefCell<TreeNode>>>, target_sum: i32, path: &[i32]) -> i32 {
        let Some(node) = root else { return 0 };
        let mut path = path.to_vec();
        path.push(node.borrow().val);
        let num_paths_to_this = path
            .iter()
            .rev()
            .scan(0_i32, |sum, v| {
                *sum = (*sum).saturating_add(*v);
                Some(*sum == target_sum)
            })
            .filter(|&bool| bool)
            .count() as i32;
        num_paths_to_this
            + Self::_path_sum(&node.borrow().left, target_sum, &path)
            + Self::_path_sum(&node.borrow().right, target_sum, &path)
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
