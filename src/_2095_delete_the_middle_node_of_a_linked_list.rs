/*
 * @lc app=leetcode id=2095 lang=rust
 *
 * [2095] Delete the Middle Node of a Linked List
 */

// @lc code=start
impl Solution {
    pub fn delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut rabbit = head.clone();
        let mut turtle = head;
        let mut arr = vec![];
        while let Some(rab_1) = rabbit {
            let Some(rab_2) = rab_1.next else {
                break;
            };
            let tur = turtle.unwrap();
            arr.push(tur.val);
            turtle = tur.next;
            rabbit = rab_2.next;
        }
        let mut ans = turtle.map(|n| n.next).unwrap_or_default();
        for val in arr.into_iter().rev() {
            ans = Some(Box::new(ListNode { val, next: ans }))
        }
        ans
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
