/*
 * @lc app=leetcode id=55 lang=rust
 *
 * [55] Jump Game
 */

// @lc code=start
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let n = nums.len();
        if n == 1 {
            return true;
        }
        if nums[0] == 0 {
            return false;
        }
        let mut r = n - 1;
        while 0 < r {
            if nums[r] == 0 && r < n - 1 {
                let mut rr = r;
                while r - rr >= nums[rr] as usize {
                    rr -= 1;
                    if rr == 0 {
                        return r < nums[0] as usize;
                    }
                }
                r = rr;
            }
            r -= 1;
        }
        true
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
            (vec![2, 3, 1, 1, 4], true),
            (vec![3, 2, 1, 0, 4], false),
            (vec![4, 2, 1, 0, 4], true),
            (vec![3, 3, 1, 0, 4], true),
            (vec![0, 1], false),
            (vec![2, 0, 0], true),
            (vec![1, 1, 1, 0], true),
        ];
        for (args, ret) in cases {
            assert_eq!(
                super::Solution::can_jump(args.clone()),
                ret,
                "case: {args:?}, {ret:?}"
            )
        }
    }
}
