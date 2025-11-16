#
# @lc app=leetcode id=209 lang=python3
#
# [209] Minimum Size Subarray Sum
#


# @lc code=start
class Solution:
    def minSubArrayLen(self, target: int, nums: list[int]) -> int:
        ans = float("inf")
        l = 0
        r = 0
        s = nums[0]
        while r < len(nums):
            if s < target:
                r += 1
                try:
                    s += nums[r]
                except IndexError:
                    break
            else:
                ans = min(ans, r - l + 1)
                s -= nums[l]
                l += 1
        return ans if ans != float("inf") else 0


# @lc code=end

s = Solution()
assert s.minSubArrayLen(target=7, nums=[2, 3, 1, 2, 4, 3]) == 2
assert s.minSubArrayLen(target=4, nums=[1, 4, 4]) == 1
assert s.minSubArrayLen(target=11, nums=[1, 1, 1, 1, 1, 1, 1, 1]) == 0
