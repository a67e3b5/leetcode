/*
 * @lc app=leetcode id=33 lang=typescript
 *
 * [33] Search in Rotated Sorted Array
 */

// @lc code=start
function search(nums: number[], target: number): number {
    let l = 0;
    let r = nums.length - 1;
    while (l <= r) {
        const mid = Math.floor((l + r) / 2);
        if (nums[mid] === target) {
            return mid
        } else if (nums[mid]! >= nums[l]!) {
            if (nums[l]! <= target && target <= nums[mid]!) {
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        } else {
            if (nums[mid]! <= target && target <= nums[r]!) {
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }
    }
    return -1;
};
// @lc code=end

console.log(search([1, 3], 1));
