/*
 * @lc app=leetcode id=2390 lang=rust
 *
 * [2390] Removing Stars From a String
 */

// @lc code=start
impl Solution {
    pub fn remove_stars(s: String) -> String {
        let s = s.as_bytes();
        let mut rez = vec![0; s.len()];
        let mut i = 0;
        for b in s {
            match *b {
                b'*' => i -= 1,
                c => {
                    rez[i] = c;
                    i += 1;
                }
            }
        }
        rez.into_iter().take(i).map(|b| b as char).collect()
    }
}
// @lc code=end

struct Solution;
