/*
 * @lc app=leetcode id=20 lang=kotlin
 *
 * [20] Valid Parentheses
 */

// @lc code=start
class Solution {
    fun isValid(s: String): Boolean {
        val stack = ArrayDeque<Char>()
        val pairs = mapOf(')' to '(', ']' to '[', '}' to '{')
        for (c in s) {
            val open = pairs[c]
            if (open != null) {
                if (stack.isEmpty() || stack.removeLast() != open) return false
            } else {
                stack.addLast(c)
            }
        }
        return stack.isEmpty()
    }
}
// @lc code=end
