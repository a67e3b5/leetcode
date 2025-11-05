/*
 * @lc app=leetcode id=435 lang=rust
 *
 * [435] Non-overlapping Intervals
 */

// @lc code=start
impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals
            .into_iter()
            .map(|v| (v[0], v[1]))
            .collect::<Vec<_>>();
        intervals.sort_unstable_by_key(|(l, _)| *l);
        let mut ans = 0;
        let mut last = i32::MIN;
        for (l, r) in intervals {
            if l < last {
                last = r.min(last);
                ans += 1;
            } else {
                last = r;
            }
        }
        ans
    }
}
// @lc code=end

struct Solution;
