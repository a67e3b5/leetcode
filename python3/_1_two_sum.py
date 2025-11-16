#
# @lc app=leetcode id=1 lang=python3
#
# [1] Two Sum
#


# @lc code=start
class Solution:
    def twoSum(self, nums: list[int], target: int) -> list[int]:
        index = {}
        for i, num in enumerate(nums):
            t = target - num
            if t in index:
                return [i, index[t]]
            index[num] = i


# @lc code=end
