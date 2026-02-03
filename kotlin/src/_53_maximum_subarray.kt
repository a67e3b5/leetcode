/*
 * @lc app=leetcode id=53 lang=kotlin
 *
 * [53] Maximum Subarray
 */

// @lc code=start
class Solution {
    fun maxSubArray(nums: IntArray): Int {
        var maxSum = nums[0]
        var curSum = nums[0]
        for (i in 1 until nums.size) {
            curSum = maxOf(nums[i], curSum + nums[i])
            maxSum = maxOf(maxSum, curSum)
        }
        return maxSum
    }
}
// @lc code=end
