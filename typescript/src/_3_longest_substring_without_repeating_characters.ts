/*
 * @lc app=leetcode id=3 lang=typescript
 *
 * [3] Longest Substring Without Repeating Characters
 */

// @lc code=start
function lengthOfLongestSubstring(s: string): number {
    const n_chars = s.length;
    if (n_chars < 2) {
        return n_chars;
    }
    const arr: string[] = s.split('');
    let l = 0;
    let r = 0;
    let max_len = 0;
    while (r < n_chars) {
        let i = arr.slice(l, r).indexOf(arr[r]!);
        if (-1 < i) {
            max_len = Math.max(max_len, r - l);
            l += i + 1;
            r += 1;
        } else {
            r += 1;
        }
    }
    max_len = Math.max(max_len, r - l);
    return max_len
};
// @lc code=end
