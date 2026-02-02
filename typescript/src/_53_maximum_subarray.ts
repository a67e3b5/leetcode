/*
 * @lc app=leetcode id=53 lang=typescript
 *
 * [53] Maximum Subarray
 */

// @lc code=start
function maxSubArray(nums: number[]): number {
    let maxSum = nums[0]!;
    let curSum = nums[0]!;
    for (let i = 1; i < nums.length; i++) {
        curSum = Math.max(nums[i]!, curSum + nums[i]!);
        maxSum = Math.max(maxSum, curSum);
    }
    return maxSum;
};
// @lc code=end
