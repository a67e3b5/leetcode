/*
 * @lc app=leetcode id=27 lang=rust
 *
 * [27] Remove Element
 */

// @lc code=start
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }
        let mut l = 0;
        let mut r = n - 1;
        while l < r {
            while nums[r] == val && l < r {
                r -= 1;
            }
            if nums[l] == val {
                nums[l] = nums[r];
                nums[r] = val;
            }
            l += 1;
        }
        r as i32 + (nums[r] != val).then_some(1).unwrap_or(0)
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let cases = [
            (vec![], 0, 0),
            (vec![1], 0, 1),
            (vec![1], 1, 0),
            (vec![3, 3], 3, 0),
            (vec![3, 3, 3, 3], 3, 0),
            (vec![3, 3], 5, 2),
            (vec![3, 3, 3, 3], 5, 4),
            (vec![3, 2, 2, 3], 3, 2),
            (vec![0, 1, 2, 2, 3, 0, 4, 2], 2, 5),
        ];
        for (mut nums, val, ret) in cases {
            assert_eq!(
                super::Solution::remove_element(&mut nums, val),
                ret,
                "{nums:?}, {val}"
            )
        }
    }
}
