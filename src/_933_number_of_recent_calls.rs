/*
 * @lc app=leetcode id=933 lang=rust
 *
 * [933] Number of Recent Calls
 */

// @lc code=start
struct RecentCounter {
    lap_times: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
use std::collections::VecDeque;

impl RecentCounter {
    fn new() -> Self {
        Self {
            lap_times: VecDeque::new(),
        }
    }

    fn ping(&mut self, t: i32) -> i32 {
        while let Some(&s) = self.lap_times.front() {
            if t - 3000 <= s {
                break;
            }
            self.lap_times.pop_front();
        }
        self.lap_times.push_back(t);
        self.lap_times.len() as i32
    }
}

/*
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */
// @lc code=end
