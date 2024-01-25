/*
 * @lc app=leetcode id=11 lang=rust
 *
 * [11] Container With Most Water
 */

// @lc code=start
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut height = height.into_iter().map(|i| i as usize).collect::<Vec<_>>();
        let mut ans = 0;
        let mut max_height = 0;
        let mut r_idx = height.len();
        while let Some(r_h) = height.pop() {
            r_idx -= 1;
            if r_h <= max_height {
                continue; // optimization
            } else {
                max_height = r_h;
            }
            let r_skip = ans / r_h;
            let Some(max_area_in_this_r) = height
                .iter()
                .enumerate()
                .take(r_idx.checked_sub(r_skip).unwrap_or_default()) // optimization
                .scan(0, |max_h, (i, &h)| {
                    (*max_h < h)
                        .then(|| {
                            *max_h = h;
                            (r_idx - i) * h.min(r_h)
                        })
                        .or(Some(0)) // optimization
                })
                .max()
            else {
                continue;
            };
            if ans < max_area_in_this_r {
                ans = max_area_in_this_r;
            }
        }
        ans as i32
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
