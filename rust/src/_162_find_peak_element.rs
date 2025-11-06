/*
 * @lc app=leetcode id=162 lang=rust
 *
 * [162] Find Peak Element
 */

// @lc code=start
impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut prev = i32::MIN;
        for (i, num) in nums.iter().enumerate() {
            if prev > *num {
                return i as i32 - 1;
            }
            prev = *num;
        }
        nums.len() as i32 - 1
    }
}
// @lc code=end

struct Solution;

mod tests {
    use super::Solution;

    #[test]
    fn test1() {
        let nums = vec![1, 2, 3, 1];
        let res = Solution::find_peak_element(nums);
        assert_eq!(res, 2);
    }

    #[test]
    fn test2() {
        let nums = vec![1, 2, 1, 3, 5, 6, 4];
        let res = Solution::find_peak_element(nums);
        assert!(res == 1 || res == 5);
    }
}
