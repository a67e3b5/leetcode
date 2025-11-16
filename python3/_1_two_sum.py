#
# @lc app=leetcode id=1 lang=python3
#
# [1] Two Sum
#


# @lc code=start
class Solution:
    def twoSum(self, nums: list[int], target: int) -> list[int]:
        nums = sorted(enumerate(nums), key=lambda x: x[1])
        i = 0
        j = len(nums) - 1
        while i < j:
            s = nums[i][1] + nums[j][1]
            if s < target:
                i += 1
            elif s > target:
                j -= 1
            else:
                return [nums[i][0], nums[j][0]]


# @lc code=end
