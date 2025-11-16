#
# @lc app=leetcode id=167 lang=python3
#
# [167] Two Sum II - Input Array Is Sorted
#


# @lc code=start
from collections import deque


class Solution:
    def twoSum(self, numbers: list[int], target: int) -> list[int]:
        left = 0
        right = len(numbers) - 1

        while left < right:
            total = numbers[left] + numbers[right]

            if total == target:
                return [left + 1, right + 1]
            elif total > target:
                right -= 1
            else:
                left += 1


# @lc code=end

s = Solution()
assert s.twoSum(numbers=[2, 7, 11, 15], target=9) == [1, 2]
assert s.twoSum(numbers=[2, 3, 4], target=6) == [1, 3]
assert s.twoSum(numbers=[-1, 0], target=-1) == [1, 2]
assert s.twoSum([0, 0, 3, 4], 0) == [1, 2]
