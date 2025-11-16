#
# @lc app=leetcode id=189 lang=python3
#
# [189] Rotate Array
#


# @lc code=start
class Solution:
    def rotate(self, nums: list[int], k: int) -> None:
        """
        Do not return anything, modify nums in-place instead.
        """
        k %= len(nums)
        tmp = nums[-k:]
        nums[k:] = nums[:-k]
        nums[:k] = tmp


# @lc code=end

s = Solution()
nums = [1, 2, 3, 4, 5, 6, 7]
s.rotate(nums, k=3)
assert nums == [5, 6, 7, 1, 2, 3, 4]
