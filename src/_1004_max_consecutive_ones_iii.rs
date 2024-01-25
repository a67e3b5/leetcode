/*
 * @lc app=leetcode id=1004 lang=rust
 *
 * [1004] Max Consecutive Ones III
 */

// @lc code=start
impl Solution {
    pub fn longest_ones(nums: Vec<i32>, mut k: i32) -> i32 {
        let zeros_idx_remainder = nums[0] as usize;
        let bools = nums.into_iter().map(|n| 1 == n).collect::<Vec<_>>();
        let consecutive_counts = bools
            .iter()
            .zip(
                bools
                    .iter()
                    .skip(1)
                    .chain(std::iter::once(&!bools.last().unwrap())),
            )
            .map(|(curr, next)| curr ^ next)
            .fold((vec![], 0), |(mut res, mut count), is_switching| {
                count += 1;
                if is_switching {
                    res.push(count);
                    count = 0;
                }
                (res, count)
            })
            .0;
        let counts_len = consecutive_counts.len();
        let counts_0 = consecutive_counts
            .iter()
            .enumerate()
            .map(|(i, &n)| if i % 2 == zeros_idx_remainder { n } else { 0 })
            .collect::<Vec<_>>();
        let mut counts_1 = consecutive_counts
            .iter()
            .enumerate()
            .map(|(i, &n)| if i % 2 != zeros_idx_remainder { n } else { 0 })
            .collect::<Vec<_>>();
        let mut zero_filling_indices = {
            let mut priorities = counts_0
                .iter()
                .enumerate()
                .map(|(i, &n)| {
                    let p = if i % 2 == zeros_idx_remainder && 0 < i && i + 1 < counts_len {
                        counts_1[i - 1] + counts_1[i + 1] - n
                    } else {
                        0
                    };
                    (i, p)
                })
                .collect::<Vec<_>>();
            priorities.sort_by_key(|(_, p)| *p);
            priorities.into_iter().map(|(i, _)| i).collect::<Vec<_>>()
        };
        while let (Some(i), true) = (zero_filling_indices.pop(), 0 < k) {
            let filling = counts_0[i];
            if k < filling {
                continue;
            }
            k -= filling;
            counts_1[i] += filling;
        }
        counts_1
            .split(|&n| 0 == n)
            .map(|slc| slc.iter().sum())
            .max()
            .unwrap_or(0)
            + k
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
            ((vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2), 6),
            (
                (
                    vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1],
                    3,
                ),
                10,
            ),
        ];
        for (input, output) in samples {
            assert_eq!(Solution::longest_ones(input.0, input.1), output);
        }
    }
}
