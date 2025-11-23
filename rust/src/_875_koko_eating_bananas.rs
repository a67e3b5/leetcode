/*
 * @lc app=leetcode id=875 lang=rust
 *
 * [875] Koko Eating Bananas
 */

// @lc code=start
impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let is_feasible = |v: i32| h >= piles.iter().map(|p| (p - 1) / v + 1).sum::<i32>();
        let mut v_inf = 1;
        let mut v_sup = *piles.iter().max().unwrap();
        while v_inf < v_sup {
            let v_mid = (v_inf + v_sup) / 2;
            if is_feasible(v_mid) {
                v_sup = v_mid;
            } else {
                v_inf = v_mid + 1;
            }
        }
        v_inf
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let cases = [
            (vec![3, 6, 7, 11], 8, 4),
            (vec![30, 11, 23, 4, 20], 5, 30),
            (vec![30, 11, 23, 4, 20], 6, 23),
        ];
        for (piles, h, ans) in cases {
            assert_eq!(
                super::Solution::min_eating_speed(piles.clone(), h),
                ans,
                "case: {piles:?}, {h}"
            )
        }
    }
}
