/*
 * @lc app=leetcode id=547 lang=rust
 *
 * [547] Number of Provinces
 */

// @lc code=start
impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let num_cities = is_connected.len();
        let mut is_classed = vec![false; num_cities];
        let mut num_provinces = 0;
        let adjacent = |city: usize| {
            is_connected[city]
                .iter()
                .enumerate()
                .filter(|(_, &b)| b == 1)
                .map(|(i, _)| i)
        };

        for city in 0..num_cities {
            if is_classed[city] {
                continue;
            }
            num_provinces += 1;
            is_classed[city] = true;
            let mut hubs = adjacent(city)
                .filter(|&h| !is_classed[h])
                .collect::<Vec<_>>();

            while let Some(hub) = hubs.pop() {
                is_classed[hub] = true;
                hubs.extend(adjacent(hub).filter(|&h| !is_classed[h]));
            }
        }

        num_provinces
    }
}
// @lc code=end

struct Solution;
