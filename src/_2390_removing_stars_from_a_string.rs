/*
 * @lc app=leetcode id=2390 lang=rust
 *
 * [2390] Removing Stars From a String
 */

// @lc code=start
impl Solution {
    pub fn remove_stars(s: String) -> String {
        const STAR: u8 = b'*';
        let mut bs = s.into_bytes();
        let mut i = bs.len();
        let mut stars = 0;
        while 0 < i {
            i -= 1;
            let b = bs.get_mut(i).unwrap();
            if *b == STAR {
                stars += 1;
            } else if 0 < stars {
                *b = STAR;
                stars -= 1;
            }
        }
        bs.retain(|&b| b != STAR);

        String::from_utf8(bs).unwrap()
    }
}
// @lc code=end

struct Solution;
