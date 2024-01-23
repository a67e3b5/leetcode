/*
 * @lc app=leetcode id=746 lang=rust
 *
 * [746] Min Cost Climbing Stairs
 */

// @lc code=start
impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut top_height = cost.len();
        // Cost to the top from the top
        let mut c0 = 0;
        // Cost to the top from 1 step below the top
        let mut c1 = cost[top_height - 1];

        while 1 < top_height {
            let c2 = cost[top_height - 2] + c1.min(c0);
            top_height -= 1;
            c0 = c1;
            c1 = c2;
        }
        c1.min(c0)
    }
}
// @lc code=end

struct Solution;
