/*
 * @lc app=leetcode id=374 lang=rust
 *
 * [374] Guess Number Higher or Lower
 */

// @lc code=start
impl Solution {
    #[allow(non_snake_case)]
    unsafe fn guessNumber(n: i32) -> i32 {
        let mut num = 1;
        let mut shift = n - 1;
        loop {
            match guess(num) {
                0 => return num,
                1 => num += shift,
                -1 => num -= shift,
                _ => unreachable!(),
            }
            shift /= 2;
            if shift == 0 {
                shift = 1
            }
        }
    }
}
// @lc code=end

struct Solution;

/**
 * Forward declaration of guess API.
 * @param  num    your guess
 * @return        -1 if num is higher than the picked number
 *                1 if num is lower than the picked number
 *                otherwise return 0
 */
unsafe fn guess(_num: i32) -> i32 {
    unimplemented!()
}
