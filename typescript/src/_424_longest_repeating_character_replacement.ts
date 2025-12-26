/*
 * @lc app=leetcode id=424 lang=typescript
 *
 * [424] Longest Repeating Character Replacement
 */

// @lc code=start
function characterReplacement(s: string, k: number): number {
    const arr = s.split('');
    let freq: Record<string, number> = {};
    let res = 0;
    let i = 0;
    for (const [j, c] of arr.entries()) {
        if (!freq[c]) {
            freq[c] = 0;
        }
        freq[c] += 1;
        const max_freq = Math.max(...Object.values(freq));
        const cur_len = j - i + 1;
        if (cur_len - max_freq > k) {
            freq[arr[i]!]! -= 1;
            i += 1;
        }
        res = Math.max(res, j - i + 1);
    }
    return res
};
// @lc code=end
