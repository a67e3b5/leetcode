/*
 * @lc app=leetcode id=53 lang=golang
 *
 * [53] Maximum Subarray
 */

// @lc code=start
func maxSubArray(nums []int) int {
	maxSum := nums[0]
	curSum := nums[0]
	for _, n := range nums[1:] {
		if curSum < 0 {
			curSum = n
		} else {
			curSum += n
		}
		if curSum > maxSum {
			maxSum = curSum
		}
	}
	return maxSum
}

// @lc code=end
