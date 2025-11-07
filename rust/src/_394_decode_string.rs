/*
 * @lc app=leetcode id=394 lang=rust
 *
 * [394] Decode String
 */

// @lc code=start
impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut n_stack = Vec::new();
        let mut s_stack = Vec::new();
        let mut cur_n = 0_usize;
        let mut cur_s = String::new();

        for c in s.chars() {
            if c.is_ascii_digit() {
                cur_n = 10 * cur_n + (c as u8 - b'0') as usize;
            } else if c == '[' {
                n_stack.push(cur_n);
                s_stack.push(cur_s);
                cur_n = 0_usize;
                cur_s = String::new();
            } else if c == ']' {
                let repeat = n_stack.pop().unwrap();
                let prev = s_stack.pop().unwrap();
                let expanded = cur_s.repeat(repeat);
                cur_s = prev + &expanded;
            } else {
                cur_s.push(c);
            }
        }
        cur_s
    }
}
// @lc code=end

struct Solution;
