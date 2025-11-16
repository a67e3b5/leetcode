#
# @lc app=leetcode id=122 lang=python3
#
# [122] Best Time to Buy and Sell Stock II
#


# @lc code=start
class Solution:
    def maxProfit(self, prices: list[int]) -> int:
        hold = -prices[0]
        free = 0
        for p in prices:
            hold = max(hold, free - p)
            free = max(free, hold + p)
        return free


# @lc code=end

s = Solution()
assert s.maxProfit(prices=[7, 1, 5, 3, 6, 4]) == 7
assert s.maxProfit(prices=[1, 2, 3, 4, 5]) == 4
assert s.maxProfit(prices=[7, 6, 4, 3, 1]) == 0
