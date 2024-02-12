/*
 * @lc app=leetcode id=435 lang=rust
 *
 * [435] Non-overlapping Intervals
 */

// @lc code=start
impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_by_key(|v| v[0]);
        let mut ans = 0;
        let mut last = i32::MIN;
        for v in intervals {
            if v[0] < last {
                last = v[1].min(last);
                ans += 1;
            } else {
                last = v[1];
            }
        }
        ans
    }
}
// @lc code=end

struct Solution;
