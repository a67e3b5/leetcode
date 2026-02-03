/*
 * @lc app=leetcode id=242 lang=kotlin
 *
 * [242] Valid Anagram
 */

// @lc code=start
class Solution {
    fun isAnagram(s: String, t: String): Boolean {
        if (s.length != t.length) return false
        val count = IntArray(26)
        for (i in s.indices) {
            count[s[i] - 'a']++
            count[t[i] - 'a']--
        }
        return count.all { it == 0 }
    }
}
// @lc code=end
