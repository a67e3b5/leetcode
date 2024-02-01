/*
 * @lc app=leetcode id=547 lang=rust
 *
 * [547] Number of Provinces
 */

// @lc code=start
impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let num_cities = is_connected.len();
        // We name every province by its representative city, which is initialized to a non-existent city.
        let mut provinces = vec![num_cities; num_cities];

        while let Some(repr) = provinces.iter().position(|&p| p == num_cities) {
            for (j, _) in is_connected[repr]
                .iter()
                .enumerate()
                .filter(|(_, &b)| b == 1)
            {
                provinces[j] = repr
            }
        }
        provinces.sort();
        provinces.dedup();
        provinces.len() as i32
    }
}
// @lc code=end

struct Solution;
