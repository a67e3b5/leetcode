/*
 * @lc app=leetcode id=274 lang=rust
 *
 * [274] H-Index
 */

// @lc code=start
impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut freq = [0u16; 1001];
        for c in citations {
            freq[c as usize] += 1;
        }
        let mut acc = 0;
        for (i, f) in freq.iter().enumerate().rev() {
            acc += f;
            if acc as usize >= i {
                return i as i32;
            }
        }
        0
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let cases = [
            (vec![3, 0, 6, 1, 5], 3),
            (vec![1, 3, 1], 1),
            (vec![0, 2], 1),
            (vec![2], 1),
            (vec![0], 0),
        ];
        for (args, ret) in cases {
            assert_eq!(
                super::Solution::h_index(args.clone()),
                ret,
                "case: {args:?}, {ret:?}"
            )
        }
    }
}
