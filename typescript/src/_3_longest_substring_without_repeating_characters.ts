/*
 * @lc app=leetcode id=3 lang=typescript
 *
 * [3] Longest Substring Without Repeating Characters
 */

// @lc code=start
function lengthOfLongestSubstring(s: string): number {
    const nChars = s.length;
    if (nChars < 2) {
        return nChars;
    }
    const arr: string[] = s.split('');
    let l = 0;
    let r = 0;
    let maxLen = 0;
    while (r < nChars) {
        const i = arr.slice(l, r).indexOf(arr[r]!);
        if (-1 < i) {
            maxLen = Math.max(maxLen, r - l);
            l += i + 1;
            r += 1;
        } else {
            r += 1;
        }
    }
    maxLen = Math.max(maxLen, r - l);
    return maxLen
};
// @lc code=end
