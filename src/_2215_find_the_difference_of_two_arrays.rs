/*
 * @lc app=leetcode id=2215 lang=rust
 *
 * [2215] Find the Difference of Two Arrays
 */

// @lc code=start
impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let idx = |x: i32| -> usize { (x + 1000) as _ };
        let mut counter = [0_u8; 2001];
        nums1.iter().for_each(|&x| counter[idx(x)] |= 0b01);
        nums2.iter().for_each(|&x| counter[idx(x)] |= 0b10);

        let mut res = vec![Vec::new(), Vec::new()];
        counter.iter().enumerate().for_each(|(i, &x)| match x {
            0b01 => res[0].push(i as i32 - 1000),
            0b10 => res[1].push(i as i32 - 1000),
            _ => (),
        });
        res
    }
}
// @lc code=end

struct Solution;
