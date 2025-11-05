/*
 * @lc app=leetcode id=2095 lang=rust
 *
 * [2095] Delete the Middle Node of a Linked List
 */

// @lc code=start
impl Solution {
    pub fn delete_middle(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut num_nodes: usize = 1;
        let mut ptr = head.as_ref()?.as_ref();
        while let Some(boxed_node) = ptr.next.as_ref() {
            ptr = boxed_node.as_ref();
            num_nodes += 1;
        }

        if num_nodes == 1 {
            return None;
        }

        let mut base: &mut ListNode = head.as_mut().unwrap().as_mut();
        for _ in 0..(num_nodes >> 1) - 1 {
            base = base.next.as_mut().unwrap().as_mut();
        }

        base.next = base.next.as_mut().unwrap().as_mut().next.take();
        head
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let samples = [
            (vec![1, 3, 4, 7, 1, 2, 6], vec![1, 3, 4, 1, 2, 6]),
            (vec![1, 2, 3, 4], vec![1, 2, 4]),
            (vec![2, 1], vec![2]),
            (vec![], vec![]),
        ];
        for (input, output) in samples {
            let input = input.try_into().ok().map(Box::new);
            let output = output.try_into().ok().map(Box::new);

            assert_eq!(Solution::delete_middle(input), output);
        }
    }

    impl TryFrom<Vec<i32>> for ListNode {
        type Error = &'static str;

        fn try_from(mut src: Vec<i32>) -> Result<Self, Self::Error> {
            if src.is_empty() {
                return Err("empty src");
            }
            let mut res = ListNode::new(src.pop().unwrap());
            for val in src.into_iter().rev() {
                res = ListNode {
                    val,
                    next: Some(Box::new(res)),
                }
            }
            Ok(res)
        }
    }
}
