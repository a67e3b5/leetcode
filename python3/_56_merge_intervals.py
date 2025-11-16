#
# @lc app=leetcode id=56 lang=python3
#
# [56] Merge Intervals
#


# @lc code=start
class Solution:
    def merge(self, intervals: list[list[int]]) -> list[list[int]]:
        if len(intervals) == 1:
            return intervals
        intervals = sorted(intervals, key=lambda x: x[0])
        ans = []
        last = intervals[0]
        for interval in intervals[1:]:
            if last[1] < interval[0]:
                ans.append(last)
                last = interval
            else:
                last[1] = max(last[1], interval[1])
        ans.append(last)
        return ans


# @lc code=end
