/*
 * @lc app=leetcode id=841 lang=rust
 *
 * [841] Keys and Rooms
 */

// @lc code=start
impl Solution {
    pub fn can_visit_all_rooms(mut rooms: Vec<Vec<i32>>) -> bool {
        let num_rooms = rooms.len();
        let mut is_open = vec![false; num_rooms];
        is_open[0] = true;
        let mut keys = rooms[0].clone();

        while let Some(key) = keys.pop() {
            let key = key as usize;
            if !is_open[key] {
                is_open[key] = true;
                keys.append(&mut rooms[key]);
            }
        }

        is_open.into_iter().all(core::convert::identity)
    }
}
// @lc code=end

struct Solution;
