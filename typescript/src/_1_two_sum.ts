/*
 * @lc app=leetcode id=1 lang=typescript
 *
 * [1] Two Sum
 */

// @lc code=start
function twoSum(nums: number[], target: number): number[] {
    const map = new Map<number, number>();
    for (let i = 0; i < nums.length; i++) {
        const j = map.get(target - nums[i]!);
        if (j !== undefined) {
            return [j, i];
        }
        map.set(nums[i]!, i);
    }
    return [];
};
// @lc code=end
