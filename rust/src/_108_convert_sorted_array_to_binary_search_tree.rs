/*
 * @lc app=leetcode id=108 lang=rust
 *
 * [108] Convert Sorted Array to Binary Search Tree
 */

// @lc code=start

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::solve(&nums)
    }

    fn solve(range: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        let n = range.len();

        if n == 0 {
            return None;
        }
        if n == 1 {
            return Some(Rc::new(RefCell::new(TreeNode::new(range[0]))));
        }

        let (left_range, right_range) = range.split_at(n / 2);
        let val = right_range[0];
        let left = Self::solve(left_range);
        let right = Self::solve(&right_range[1..]);

        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
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
