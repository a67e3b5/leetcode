#
# @lc app=leetcode id=167 lang=python3
#
# [167] Two Sum II - Input Array Is Sorted
#


# @lc code=start
class Solution:
    def twoSum(self, numbers: list[int], target: int) -> list[int]:
        for i, m in list(enumerate(numbers))[::-1]:
            if m > target:
                continue
            t = target - m
            for j, n in enumerate(numbers):
                if n == t:
                    return [j + 1, i + 1]


# @lc code=end
