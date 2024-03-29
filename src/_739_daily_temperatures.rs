/*
 * @lc app=leetcode id=739 lang=rust
 *
 * [739] Daily Temperatures
 */

// @lc code=start
impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0_i32; temperatures.len()];
        let mut stack: Vec<(usize, i32)> = vec![];
        for (i, t) in temperatures.into_iter().enumerate().rev() {
            while !stack.is_empty() && stack.last().unwrap().1 <= t {
                stack.pop();
            }
            if !stack.is_empty() {
                ans[i] = (stack.last().unwrap().0 - i) as i32;
            }
            stack.push((i, t));
        }
        ans
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let samples = [(
            vec![89, 62, 70, 58, 47, 47, 46, 76, 100, 70],
            vec![8, 1, 5, 4, 3, 2, 1, 1, 0, 0],
        )];
        for (input, output) in samples {
            assert_eq!(Solution::daily_temperatures(input), output);
        }
    }
}
