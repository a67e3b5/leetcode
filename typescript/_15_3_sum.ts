/*
 * @lc app=leetcode id=15 lang=typescript
 *
 * [15] 3Sum
 */

// @lc code=start
function threeSum(nums: number[]): number[][] {
    let res: number[][] = [];
    nums.sort((a, b) => a - b);

    for (const i of new Array(nums.length).keys()) {
        if (i > 0 && nums[i] === nums[i - 1]) {
            continue;
        }
        let j = i + 1;
        let k = nums.length - 1;
        while (j < k) {
            let total = nums[i] + nums[j] + nums[k];
            if (total > 0) {
                k -= 1;
            } else if (total < 0) {
                j += 1;
            } else {
                res.push([nums[i], nums[j], nums[k]]);
                j += 1;
                while (nums[j] === nums[j - 1] && j < k) {
                    j += 1;
                }
            }
        }
    }
    return res
};
// @lc code=end
