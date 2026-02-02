/*
 * @lc app=leetcode id=20 lang=typescript
 *
 * [20] Valid Parentheses
 */

// @lc code=start
function isValid(s: string): boolean {
    const stack: string[] = [];
    const pairs = new Map([[')', '('], [']', '['], ['}', '{']]);
    for (const c of s) {
        const open = pairs.get(c);
        if (open !== undefined) {
            if (stack.pop() !== open) return false;
        } else {
            stack.push(c);
        }
    }
    return stack.length === 0;
};
// @lc code=end
