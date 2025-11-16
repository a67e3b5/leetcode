#
# @lc app=leetcode id=209 lang=python3
#
# [209] Minimum Size Subarray Sum
#


# @lc code=start
class Solution:
    def minSubArrayLen(self, target: int, nums: list[int]) -> int:
        ans = 0
        l = 0
        r = 0
        while r <= len(nums):
            s = sum(nums[l:r])
            if s < target:
                r += 1
            elif s > target:
                l += 1
            elif ans == 0:
                ans = r - l
                r += 1
            else:
                ans = min(ans, r - l)
                r += 1

        return ans


# @lc code=end

s = Solution()
assert s.minSubArrayLen(target=7, nums=[2, 3, 1, 2, 4, 3]) == 2
assert s.minSubArrayLen(target=4, nums=[1, 4, 4]) == 1
assert s.minSubArrayLen(target=11, nums=[1, 1, 1, 1, 1, 1, 1, 1]) == 0
