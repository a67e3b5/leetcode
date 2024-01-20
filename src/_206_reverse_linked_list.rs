/*
 * @lc app=leetcode id=206 lang=rust
 *
 * [206] Reverse Linked List
 */

// @lc code=start
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::_internal(head, None)
    }
    fn _internal(head: Option<Box<ListNode>>, out: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if let Some(node) = head {
            let out = Some(Box::new(ListNode {
                val: node.val,
                next: out,
            }));
            Self::_internal(node.next, out)
        } else {
            out
        }
    }
}
// @lc code=end

struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}
