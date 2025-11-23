/*
 * @lc app=leetcode id=739 lang=rust
 *
 * [739] Daily Temperatures
 */

// @lc code=start
impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut arr = [usize::MAX; 71];
        let mut ans = vec![0; temperatures.len()];
        for (i, &t) in temperatures.iter().enumerate().rev() {
            let t = t as usize - 30;
            arr[t] = i;
            if let Some(j) = arr[t + 1..].iter().filter(|&&j| j < usize::MAX).min() {
                ans[i] = (j - i) as i32;
            }
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
