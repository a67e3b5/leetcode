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
        Self::_path_sum(&root, target_sum, vec![].as_mut())
    }
    pub fn _path_sum(
        root: &Option<Rc<RefCell<TreeNode>>>,
        target_sum: i32,
        sums: &mut Vec<i32>,
    ) -> i32 {
        let Some(rc_node) = root else { return 0 };
        let node = rc_node.borrow();
        let mut count = 0;
        sums.push(0);
        sums.iter_mut().for_each(|x| {
            *x = x.saturating_add(node.val);
            if *x == target_sum {
                count += 1;
            }
        });
        count = count
            + Self::_path_sum(&node.left, target_sum, sums)
            + Self::_path_sum(&node.right, target_sum, sums);
        sums.pop();
        sums.iter_mut()
            .for_each(|x| *x = x.saturating_sub(node.val));
        count
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
