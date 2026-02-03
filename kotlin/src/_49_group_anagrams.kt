/*
 * @lc app=leetcode id=49 lang=kotlin
 *
 * [49] Group Anagrams
 */

// @lc code=start
class Solution {
    fun groupAnagrams(strs: Array<String>): List<List<String>> {
        val groups = mutableMapOf<String, MutableList<String>>()
        for (s in strs) {
            val key = String(s.toCharArray().also { it.sort() })
            groups.getOrPut(key) { mutableListOf() }.add(s)
        }
        return groups.values.toList()
    }
}
// @lc code=end
