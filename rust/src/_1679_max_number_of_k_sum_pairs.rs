/*
 * @lc app=leetcode id=1679 lang=rust
 *
 * [1679] Max Number of K-Sum Pairs
 */

// @lc code=start
impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut indices: Vec<usize> = (0..nums.len()).collect();
        let mut count = 0;
        loop {
            let mut remove = vec![];
            'a: for (i, ii) in indices.iter().enumerate() {
                for (j, jj) in indices.iter().enumerate().skip_while(|(j, _)| *j <= i) {
                    if nums[*ii] + nums[*jj] == k {
                        remove.push(j);
                        remove.push(i);
                        break 'a;
                    }
                }
            }
            if remove.is_empty() {
                break;
            }
            for i in remove {
                indices.remove(i);
            }
            count += 1;
        }
        count
    }
}
// @lc code=end

struct Solution;
