/*
 * @lc app=leetcode id=735 lang=rust
 *
 * [735] Asteroid Collision
 */

// @lc code=start
use std::cmp::Ordering;

impl Solution {
    pub fn asteroid_collision(mut asteroids: Vec<i32>) -> Vec<i32> {
        let mut positives: Vec<i32> = vec![];
        let mut negatives: Vec<i32> = vec![];
        while let Some(a0) = asteroids.pop() {
            if a0.is_negative() {
                negatives.push(a0);
                continue;
            }
            let Some(a1) = negatives.pop() else {
                positives.push(a0);
                continue;
            };
            match (a0).cmp(&-a1) {
                Ordering::Less => {
                    negatives.push(a1);
                }
                Ordering::Equal => {}
                Ordering::Greater => {
                    asteroids.push(a0);
                }
            }
        }
        positives.append(&mut negatives);
        positives.reverse();
        positives
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let samples = [(vec![2, -1, 3, -2, -4, -3], vec![-4, -3])];
        for (input, output) in samples {
            assert_eq!(Solution::asteroid_collision(input), output);
        }
    }
}
