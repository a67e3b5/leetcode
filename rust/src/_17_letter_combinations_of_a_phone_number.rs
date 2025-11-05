/*
 * @lc app=leetcode id=17 lang=rust
 *
 * [17] Letter Combinations of a Phone Number
 */

// @lc code=start
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }
        let map: Vec<(u8, u8)> = digits
            .chars()
            .map(|d| match d {
                '2' => (b'a', 3),
                '3' => (b'd', 3),
                '4' => (b'g', 3),
                '5' => (b'j', 3),
                '6' => (b'm', 3),
                '7' => (b'p', 4),
                '8' => (b't', 3),
                '9' => (b'w', 4),
                _ => unreachable!(),
            })
            .collect();
        let mut ans: Vec<Vec<u8>> = vec![vec![]];
        for (base, num) in map {
            let mut words = vec![];
            for w in ans {
                for i in 0..num {
                    let mut word = w.clone();
                    word.push(base + i);
                    words.push(word);
                }
            }
            ans = words
        }
        ans.into_iter()
            .map(|v| String::from_utf8(v).unwrap())
            .collect()
    }
}
// @lc code=end

struct Solution;
