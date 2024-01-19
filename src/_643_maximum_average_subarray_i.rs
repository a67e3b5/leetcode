/*
 * @lc app=leetcode id=643 lang=rust
 *
 * [643] Maximum Average Subarray I
 */

// @lc code=start
impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut max = f64::MIN;
        for offset in 0..=(nums.len() - k as usize) {
            let array = nums.get(offset..offset + k as usize).unwrap();
            let avg: f64 = array.iter().sum::<i32>() as f64 / k as f64;
            if max < avg {
                max = avg;
            }
        }
        max
    }
}
// @lc code=end

struct Solution;

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test() {
//         let samples = [("abc", "ahbgdc", true), ("axc", "ahbgdc", false)];
//         for (sub, sup, res) in samples {
//             assert_eq!(
//                 Solution::is_subsequence(sub.to_string(), sup.to_string()),
//                 res
//             );
//         }
//     }
// }
