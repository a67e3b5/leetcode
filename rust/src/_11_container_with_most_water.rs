/*
 * @lc app=leetcode id=11 lang=rust
 *
 * [11] Container With Most Water
 */

// @lc code=start
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut iter = height.iter().enumerate();
        let mut p1 = iter.next();
        let mut p2 = iter.next_back();
        while let (Some((i, h1)), Some((j, h2))) = (p1, p2) {
            result = result.max(h1.min(h2) * (j - i) as i32);
            if h1 < h2 {
                p1 = iter.next();
            } else {
                p2 = iter.next_back();
            }
        }
        result
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let samples = [
            (vec![2, 3, 10, 5, 7, 8, 9], 36),
            (vec![1, 3, 2, 5, 25, 24, 5], 24),
        ];
        for (input, output) in samples {
            assert_eq!(Solution::max_area(input), output);
        }
    }
}
