/*
 * @lc app=leetcode id=20 lang=rust
 *
 * [20] Valid Parentheses
 */

// @lc code=start
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let is_close = |c: char| ")]}".contains(c);
        let counterpart = |c: char| match c {
            ')' => '(',
            ']' => '[',
            '}' => '{',
            _ => unreachable!(),
        };
        let mut stack = Vec::new();
        for c in s.chars() {
            if is_close(c) {
                let Some(_open) = stack.pop_if(|o| *o == counterpart(c)) else {
                    return false;
                };
            } else {
                stack.push(c);
            }
        }
        stack.is_empty()
    }
}
// @lc code=end

struct Solution;
