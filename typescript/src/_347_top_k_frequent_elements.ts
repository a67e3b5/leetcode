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
    const freqToNum = new Array<[number, number]>();
    for (const [n, f] of numToFreq.entries()) {
        freqToNum.push([f, n]);
    }
    return freqToNum
        .sort((a, b) => b[0] - a[0])
        .slice(0, k)
        .map((v, _i, _a) => v[1])
};
// @lc code=end

console.log(topKFrequent([1, 1, 1, 2, 2, 3], 2));
// [1, 2]: number[]
console.log(topKFrequent([1, 1, 1, 1, 2, 2, 2, 2, 3, 3], 2));
// [1, 2]
console.log(topKFrequent([5, 1, -1, -8, -7, 8, -5, 0, 1, 10, 8, 0, -4, 3, -1, -1, 4, -5, 4, -3, 0, 2, 2, 2, 4, -2, -4, 8, -7, -7, 2, -8, 0, -8, 10, 8, -8, -2, -9, 4, -7, 6, 6, -1, 4, 2, 8, -3, 5, -9, -3, 6, -8, -5, 5, 10, 2, -5, -1, -5, 1, -3, 7, 0, 8, -2, -3, -1, -5, 4, 7, -9, 0, 2, 10, 4, 4, -4, -1, -1, 6, -8, -9, -1, 9, -9, 3, 5, 1, 6, -1, -2, 4, 2, 4, -6, 4, 4, 5, -5], 7));
// [4, -1, 2, -5, -8, 8, 0]
