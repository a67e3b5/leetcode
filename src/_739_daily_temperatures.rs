/*
 * @lc app=leetcode id=739 lang=rust
 *
 * [739] Daily Temperatures
 */

// @lc code=start
impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let n = temperatures.len();
        let mut ans = vec![0; n];
        let mut stack = Vec::new();
        for i in (0..n).rev() {
            while !stack.is_empty() && temperatures[i] >= temperatures[*stack.last().unwrap()] {
                stack.pop();
            }
            ans[i] = if !stack.is_empty() {
                *stack.last().unwrap() as i32 - i as i32
            } else {
                0
            };
            stack.push(i);
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
