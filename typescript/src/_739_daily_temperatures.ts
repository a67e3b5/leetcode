/*
 * @lc app=leetcode id=739 lang=typescript
 *
 * [739] Daily Temperatures
 */

// @lc code=start
function dailyTemperatures(temperatures: number[]): number[] {
    const ans = Array<number>(temperatures.length).fill(0);
    const stack: [number, number][] = [];
    temperatures.reverse().forEach((t, i, _arr) => {
        while (stack.length !== 0 && stack[-1]![1] <= t) {
            stack.pop();
        }
        if (stack.length !== 0) {
            ans[i] = stack[-1]![0] - i;
        }
        stack.push([i, t]);
    });
    return ans
};
// @lc code=end

console.log(dailyTemperatures([89, 62, 70, 58, 47, 47, 46, 76, 100, 70]));
// [8, 1, 5, 4, 3, 2, 1, 1, 0, 0]
