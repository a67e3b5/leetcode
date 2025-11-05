/*
 * @lc app=leetcode id=328 lang=rust
 *
 * [328] Odd Even Linked List
 */

// @lc code=start
impl Solution {
    pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut odd_node = ListNode::new(-1);
        let mut even_node = ListNode::new(-1);
        let mut odd_ptr = &mut odd_node;
        let mut even_ptr = &mut even_node;
        let mut is_odd = true;

        while let Some(mut node) = head {
            head = node.next.take();
            if is_odd {
                odd_ptr.next = Some(node);
                odd_ptr = odd_ptr.next.as_mut().unwrap();
            } else {
                even_ptr.next = Some(node);
                even_ptr = even_ptr.next.as_mut().unwrap();
            }
            is_odd = !is_odd;
        }

        odd_ptr.next = even_node.next;
        odd_node.next
    }
}
// @lc code=end

struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
