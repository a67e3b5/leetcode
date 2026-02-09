#
# @lc app=leetcode id=8 lang=python3
#
# [8] String to Integer (atoi)
#

# @lc code=start
class Solution:
    def myAtoi(self, s: str) -> int:
        s = s.lstrip()
        if not s:
            return 0
        sign = 1
        i = 0
        if s[0] in "+-":
            sign = -1 if s[0] == "-" else 1
            i = 1
        result = 0
        while i < len(s) and s[i].isdigit():
            result = result * 10 + int(s[i])
            i += 1
        result *= sign
        return max(-(2**31), min(2**31 - 1, result))


# @lc code=end
