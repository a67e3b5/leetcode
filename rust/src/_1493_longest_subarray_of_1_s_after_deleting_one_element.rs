/*
 * @lc app=leetcode id=1493 lang=rust
 *
 * [1493] Longest Subarray of 1's After Deleting One Element
 */

// @lc code=start
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        nums.chunk_by(|&a, &b| a == b)
            .scan((0, 0), |(p, c), ch| {
                if ch[0] == 1 {
                    *c -= *p;
                    *p = *c;
                    *c += ch.len() as i32;
                } else if ch.len() > 1 {
                    *c = 0;
                    *p = 0;
                }
                Some(*c)
            })
            .max()
            .map(|m| m - (m == nums.len() as i32) as i32)
            .unwrap()
    }
}
// @lc code=end

struct Solution;

mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::longest_subarray(vec![1, 1, 0, 1]), 3);
        assert_eq!(Solution::longest_subarray(vec![0, 0, 0, 0, 0]), 0);
        assert_eq!(Solution::longest_subarray(vec![0, 1, 1, 1, 1]), 4);
        assert_eq!(Solution::longest_subarray(vec![1, 1, 1, 1, 1]), 4);
        assert_eq!(Solution::longest_subarray(vec![0, 1, 1, 1, 0]), 3);
        assert_eq!(Solution::longest_subarray(vec![1, 1, 1, 1, 0]), 4);
        assert_eq!(Solution::longest_subarray(vec![0, 1, 0, 1, 0]), 2);
        assert_eq!(Solution::longest_subarray(vec![1, 0, 1, 0, 1]), 2);
        assert_eq!(Solution::longest_subarray(vec![0, 1, 0, 0, 1]), 1);
    }
}
