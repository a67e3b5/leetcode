/*
 * @lc app=leetcode id=217 lang=kotlin
 *
 * [217] Contains Duplicate
 */

// @lc code=start
class Solution {
    fun containsDuplicate(nums: IntArray): Boolean {
        return nums.toSet().size != nums.size
    }
}
// @lc code=end
