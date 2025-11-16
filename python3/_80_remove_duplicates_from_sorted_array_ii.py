#
# @lc app=leetcode id=80 lang=python3
#
# [80] Remove Duplicates from Sorted Array II
#


# @lc code=start
class Solution:
    def removeDuplicates(self, nums: list[int]) -> int:
        l = len(nums)
        if l < 2:
            return l
        write = 2
        for read in range(2, l):
            if nums[read] != nums[write - 2]:
                nums[write] = nums[read]
                write += 1
        return write


# @lc code=end

# nums = [0]
# nums = [0, 1]
# nums = [1, 1, 1, 2, 2, 3]
nums = [0, 0, 1, 1, 1, 1, 2, 3, 3]
sol = Solution()
print(sol.removeDuplicates(nums), nums)
