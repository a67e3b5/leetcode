/*
 * @lc app=leetcode id=567 lang=rust
 *
 * [567] Permutation in String
 */

// @lc code=start
impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let inc = |c: char, arr: &mut [i32; 26], d: i32| arr[(c as u8 - b'a') as usize] += d;
        let mut reference = [0; 26];
        s1.chars().for_each(|c| inc(c, &mut reference, 1));
        let w = s1.len();
        if w > s2.len() {
            return false;
        }
        let s2: Vec<char> = s2.chars().collect();
        let mut freq = [0; 26];
        s2[..w - 1].iter().for_each(|&c| inc(c, &mut freq, 1));
        let mut i = 0;
        while i + w < s2.len() {
            inc(s2[i + w - 1], &mut freq, 1);
            if freq == reference {
                return true;
            }
            inc(s2[i], &mut freq, -1);
            i += 1;
        }
        inc(s2[s2.len() - 1], &mut freq, 1);
        if freq == reference {
            return true;
        }
        false
    }
}
// @lc code=end

struct Solution;
