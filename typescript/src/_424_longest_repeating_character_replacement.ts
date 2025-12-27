/*
 * @lc app=leetcode id=424 lang=typescript
 *
 * [424] Longest Repeating Character Replacement
 */

// @lc code=start
function characterReplacement(s: string, k: number): number {
    const arr = s.split('');
    const freq: Record<string, number> = {};
    let res = 0;
    let i = 0;
    for (const [j, c] of arr.entries()) {
        freq[c] ??= 0;
        freq[c] += 1;
        const maxFreq = Math.max(...Object.values(freq));
        const curLen = j - i + 1;
        if (curLen - maxFreq > k) {
            freq[arr[i]!]! -= 1;
            i += 1;
        }
        res = Math.max(res, j - i + 1);
    }
    return res
};
// @lc code=end
