/*
 * @lc app=leetcode id=2336 lang=rust
 *
 * [2336] Smallest Number in Infinite Set
 */

// @lc code=start
use std::cmp::Reverse;
use std::collections::BinaryHeap;
struct SmallestInfiniteSet {
    cur: i32,
    back: BinaryHeap<Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SmallestInfiniteSet {
    fn new() -> Self {
        Self {
            cur: 0,
            back: BinaryHeap::new(),
        }
    }

    fn pop_smallest(&mut self) -> i32 {
        if let Some(Reverse(n)) = self.back.pop() {
            n
        } else {
            self.cur += 1;
            self.cur
        }
    }

    fn add_back(&mut self, num: i32) {
        if num <= self.cur && !self.back.iter().any(|&Reverse(n)| n == num) {
            self.back.push(Reverse(num))
        }
    }
}

/*
 * Your SmallestInfiniteSet object will be instantiated and called as such:
 * let obj = SmallestInfiniteSet::new();
 * let ret_1: i32 = obj.pop_smallest();
 * obj.add_back(num);
 */
// @lc code=end
