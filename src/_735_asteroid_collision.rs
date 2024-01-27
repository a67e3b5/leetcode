/*
 * @lc app=leetcode id=735 lang=rust
 *
 * [735] Asteroid Collision
 */

// @lc code=start
use std::cmp::Ordering;

impl Solution {
    pub fn asteroid_collision(mut asteroids: Vec<i32>) -> Vec<i32> {
        let mut negatives: Vec<i32> = vec![];
        while let Some(a0) = asteroids.pop() {
            if a0.is_negative() {
                negatives.push(a0);
                continue;
            }
            let Some(a1) = negatives.pop() else {
                asteroids.push(a0);
                continue;
            };
            match (a0).cmp(&-a1) {
                Ordering::Less => {
                    negatives.push(a1);
                }
                Ordering::Equal => {}
                Ordering::Greater => {
                    asteroids.push(a0);
                    if negatives.is_empty() {
                        return asteroids;
                    }
                }
            }
        }
        negatives.reverse();
        negatives
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
