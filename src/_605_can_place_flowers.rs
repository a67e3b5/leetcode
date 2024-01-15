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
        n <= counter.capacity_count
    }
}

struct Counter {
    consecutive_empties: ConsecutiveEmpties,
    capacity_count: i32,
}

enum ConsecutiveEmpties {
    Zero,
    One,
    Two,
}

impl Counter {
    fn new() -> Self {
        Self {
            consecutive_empties: ConsecutiveEmpties::Zero,
            capacity_count: 0,
        }
    }
    fn see_plot(&mut self, plot: i32) {
        match plot {
            0 => self.see_empty(),
            1 => self.see_planted(),
            _ => unreachable!(),
        }
    }
    fn see_empty(&mut self) {
        if let ConsecutiveEmpties::Two = self.consecutive_empties {
            self.capacity_count += 1;
            self.consecutive_empties.reset()
        }
        self.consecutive_empties.inc()
    }
    fn see_planted(&mut self) {
        self.consecutive_empties.reset()
    }
}

impl ConsecutiveEmpties {
    fn inc(&mut self) {
        *self = match self {
            Self::Zero => Self::One,
            Self::One => Self::Two,
            _ => unreachable!(),
        }
    }
    fn reset(&mut self) {
        *self = Self::Zero
    }
}
// @lc code=end

struct Solution;
