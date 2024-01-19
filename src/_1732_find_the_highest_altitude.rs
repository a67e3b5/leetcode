/*
 * @lc app=leetcode id=1732 lang=rust
 *
 * [1732] Find the Highest Altitude
 */

// @lc code=start
impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        std::iter::once(&0)
            .chain(gain.iter())
            .scan(0, |altitude, gain| {
                *altitude += gain;
                Some(*altitude)
            })
            .max()
            .unwrap()
    }
}
// @lc code=end

struct Solution;
