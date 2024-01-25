/*
 * @lc app=leetcode id=11 lang=rust
 *
 * [11] Container With Most Water
 */

// @lc code=start
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let hs_desc = {
            let mut res = height
                .into_iter()
                .map(|i| i as usize)
                .enumerate()
                .collect::<Vec<_>>();
            res.sort_by_key(|(_, h)| *h);
            res.reverse();
            res
        };
        let r_idx = hs_desc.len() - 1;
        let mut ans = 0;
        let mut skip = 0;

        for (i0, h0) in &hs_desc {
            skip += 1;
            if 0 == *h0 {
                continue;
            }
            let min_w = ans / h0 + 1;
            let (range_l, range_r) = (0..=(i0.saturating_sub(min_w)), (i0 + min_w)..=r_idx);
            let Some(max_area_candidate) = hs_desc
                .iter()
                .skip(skip)
                .filter(|(i, _)| range_l.contains(i) || range_r.contains(i))
                .map(|(i, h)| i.abs_diff(*i0) * h)
                .max()
            else {
                continue;
            };
            if ans < max_area_candidate {
                ans = max_area_candidate;
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
