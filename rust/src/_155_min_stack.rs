/*
 * @lc app=leetcode id=155 lang=rust
 *
 * [155] Min Stack
 */

// @lc code=start
use std::cmp::min;
use std::i32;

struct MinStack(Vec<Node>);

struct Node {
    val: i32,
    min_ever: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn push(&mut self, val: i32) {
        let min_ever = if let Some(node) = self.0.last() {
            min(node.min_ever, val)
        } else {
            val
        };
        self.0.push(Node { val, min_ever });
    }

    fn pop(&mut self) {
        self.0.pop();
    }

    fn top(&self) -> i32 {
        self.0.last().unwrap().val
    }

    fn get_min(&self) -> i32 {
        self.0.last().unwrap().min_ever
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
struct Doc;
// @lc code=end
