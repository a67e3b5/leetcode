/*
 * @lc app=leetcode id=605 lang=rust
 *
 * [605] Can Place Flowers
 */

// @lc code=start
impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut counter = Counter::new();
        counter.see_empty();
        flowerbed.iter().for_each(|i| counter.see_plot(*i));
        counter.see_empty();
        n <= counter.placable_count
    }
}

#[derive(Default)]
struct Counter {
    continuous_empties: u8,
    placable_count: i32,
}

impl Counter {
    fn new() -> Self {
        Self::default()
    }
    fn see_plot(&mut self, plot: i32) {
        match plot {
            0 => self.see_empty(),
            1 => self.see_planted(),
            _ => unreachable!(),
        }
    }
    fn see_empty(&mut self) {
        self.continuous_empties += 1;
        if 3 <= self.continuous_empties {
            self.placable_count += 1;
            self.continuous_empties = 1;
        }
    }
    fn see_planted(&mut self) {
        self.continuous_empties = 0;
    }
}
// @lc code=end

struct Solution;
