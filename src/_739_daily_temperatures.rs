/*
 * @lc app=leetcode id=739 lang=rust
 *
 * [739] Daily Temperatures
 */

// @lc code=start
impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut next_occurrence = [usize::MAX; 72]; // the last index indicates an imaginal temperature 101
        for (i, t) in temperatures.iter().enumerate().rev() {
            next_occurrence[*t as usize - 30] = i;
        }
        let mut ans = vec![0_i32; temperatures.len()];
        for (i, t) in temperatures.iter().enumerate() {
            next_occurrence[*t as usize - 30] = temperatures
                .iter()
                .skip(i + 1)
                .position(|u| u == t)
                .map_or(usize::MAX, |j| j + i + 1); // update an occurrence
            let next_warmer_idx = *next_occurrence.iter().skip(*t as usize - 29).min().unwrap();
            if next_warmer_idx < usize::MAX {
                ans[i] = (next_warmer_idx - i) as i32;
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
