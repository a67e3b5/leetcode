/*
 * @lc app=leetcode id=933 lang=rust
 *
 * [933] Number of Recent Calls
 */

// @lc code=start
struct RecentCounter {
    lap_times: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {
    fn new() -> Self {
        Self { lap_times: vec![] }
    }

    fn ping(&mut self, t: i32) -> i32 {
        self.lap_times.push(t);
        match self.lap_times.binary_search(&(t - 3000)) {
            Ok(l_edge) => (self.lap_times.len() - l_edge) as i32,
            Err(first_in_range) => (self.lap_times.len() - first_in_range) as i32,
        }
    }
}

/*
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */
// @lc code=end
