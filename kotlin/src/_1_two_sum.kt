/*
 * @lc app=leetcode id=1 lang=kotlin
 *
 * [1] Two Sum
 */

// @lc code=start
class Solution {
    fun twoSum(nums: IntArray, target: Int): IntArray {
        val map = mutableMapOf<Int, Int>()
        for ((i, n) in nums.withIndex()) {
            map[target - n]?.let { return intArrayOf(it, i) }
            map[n] = i
        }
        return intArrayOf()
    }
}
// @lc code=end
