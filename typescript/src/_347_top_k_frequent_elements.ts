/*
 * @lc app=leetcode id=347 lang=typescript
 *
 * [347] Top K Frequent Elements
 */

// @lc code=start
function topKFrequent(nums: number[], k: number): number[] {
    const numToFreq = new Map<number, number>();
    for (const n of nums) {
        numToFreq.set(n, (numToFreq.get(n) ?? 0) + 1);
    }
    const freqToNum = new Map<number, number>();
    for (const [n, f] of numToFreq.entries()) {
        freqToNum.set(f, n);
    }
    return [...freqToNum]
        .sort()
        .reverse()
        .slice(0, k)
        .map((v, _i, _a) => v[1])
};
// @lc code=end

console.log(topKFrequent([1, 1, 1, 2, 2, 3], 2));
// [1, 2]: number[]
