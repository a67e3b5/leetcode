/*
 * @lc app=leetcode id=347 lang=typescript
 *
 * [347] Top K Frequent Elements
 */

// @lc code=start
function topKFrequent(nums: number[], k: number): number[] {
    let num_to_freq: Record<number, number> = {};
    for (const n of nums) {
        if (!num_to_freq[n]) {
            num_to_freq[n] = 0;
        }
        num_to_freq[n] += 1;
    }
    let freq_to_num: Record<number, number> = {};
    for (const [n, f] of Object.entries(num_to_freq)) {
        freq_to_num[f] = n as unknown as number;
    }
    return Object
        .entries(freq_to_num)
        .map((v, i, a) => ([v[0] as unknown as number, v[1]]))
        .sort((a, b) => (b[0]! - a[0]!))
        .slice(0, k)
        .map((v, i, a) => v[1]!)
};
// @lc code=end

console.log(topKFrequent([1, 1, 1, 2, 2, 3], 2));
// ['1', '2']: string[]
