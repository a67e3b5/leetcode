#
# @lc app=leetcode id=49 lang=python3
#
# [49] Group Anagrams
#


# @lc code=start
class Solution:
    def groupAnagrams(self, strs: list[str]) -> list[list[str]]:
        dic = {}
        for s in strs:
            a = "".join(sorted(s))
            try:
                dic[a].append(s)
            except KeyError:
                dic[a] = [s]
        return list(dic.values())


# @lc code=end
s = Solution()
assert s.groupAnagrams(strs=["eat", "tea", "tan", "ate", "nat", "bat"]), [
    ["bat"],
    ["nat", "tan"],
    ["ate", "eat", "tea"],
]
assert s.groupAnagrams(strs=[""]), [[""]]
assert s.groupAnagrams(strs=["a"]), [["a"]]
