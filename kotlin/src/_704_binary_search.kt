/*
 * @lc app=leetcode id=704 lang=kotlin
 *
 * [704] Binary Search
 */

// @lc code=start
class Solution {
    fun search(nums: IntArray, target: Int): Int {
        var l = 0
        var r = nums.size - 1
        while (l <= r) {
            val mid = l + (r - l) / 2
            when {
                nums[mid] == target -> return mid
                nums[mid] < target -> l = mid + 1
                else -> r = mid - 1
            }
        }
        return -1
    }
}
// @lc code=end
