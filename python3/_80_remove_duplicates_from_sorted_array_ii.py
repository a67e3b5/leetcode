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
        num = nums[0]
        count = 1
        read = 1
        write = 0
        while read < l:
            if nums[read] == num:
                if count <= 2:
                    nums[write] = num
                    write += 1
                count += 1
            else:
                num = nums[read]
                nums[write] = num
                write += 1
                count = 1
            read += 1
        return write


# @lc code=end

# nums = [1, 1, 1, 2, 2, 3]
nums = [0, 0, 1, 1, 1, 1, 2, 3, 3]
sol = Solution()
print(sol.removeDuplicates(nums), nums)
