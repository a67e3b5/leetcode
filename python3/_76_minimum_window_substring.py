#
# @lc app=leetcode id=76 lang=python3
#
# [76] Minimum Window Substring
#

# @lc code=start
from collections import Counter


class Solution:
    def minWindow(self, s: str, t: str) -> str:
        if not t or not s:
            return ""
        need = Counter(t)
        have = {}
        required = len(need)
        formed = 0
        left = 0
        result = (float("inf"), 0, 0)

        for right, char in enumerate(s):
            have[char] = have.get(char, 0) + 1
            if char in need and have[char] == need[char]:
                formed += 1

            while formed == required:
                if right - left + 1 < result[0]:
                    result = (right - left + 1, left, right)
                left_char = s[left]
                have[left_char] -= 1
                if left_char in need and have[left_char] < need[left_char]:
                    formed -= 1
                left += 1

        return "" if result[0] == float("inf") else s[result[1] : result[2] + 1]
# @lc code=end

