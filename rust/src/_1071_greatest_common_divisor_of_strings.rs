/*
 * @lc app=leetcode id=1071 lang=rust
 *
 * [1071] Greatest Common Divisor of Strings
 */

// @lc code=start
impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let gcd = str1.len().gcd(str2.len());
        let mut str12 = str1.clone() + &str2;
        let str21 = str2 + &str1;
        if str12 != str21 {
            return String::new();
        }
        str12.truncate(gcd);
        str12
    }
}

pub trait Gcd {
    /// Determine [greatest common divisor](https://en.wikipedia.org/wiki/Greatest_common_divisor)
    /// using [`gcd_binary`].
    ///
    /// [`gcd_binary`]: #method.gcd_binary
    ///
    /// # Examples
    ///
    /// ```
    /// use gcd::Gcd;
    ///
    /// assert_eq!(0, 0u8.gcd(0));
    /// assert_eq!(10, 10u8.gcd(0));
    /// assert_eq!(10, 0u8.gcd(10));
    /// assert_eq!(10, 10u8.gcd(20));
    /// assert_eq!(44, 2024u32.gcd(748));
    /// ```
    fn gcd(self, other: Self) -> Self;

    /// Determine [greatest common divisor](https://en.wikipedia.org/wiki/Greatest_common_divisor)
    /// using the [Binary GCD algorithm](https://en.wikipedia.org/wiki/Binary_GCD_algorithm).
    fn gcd_binary(self, other: Self) -> Self;
}

///Const binary GCD implementation for `usize`.
pub const fn binary_usize(mut u: usize, mut v: usize) -> usize {
    if u == 0 {
        return v;
    }
    if v == 0 {
        return u;
    }
    let shift = (u | v).trailing_zeros();
    u >>= shift;
    v >>= shift;
    u >>= u.trailing_zeros();
    loop {
        v >>= v.trailing_zeros();
        #[allow(clippy::manual_swap)]
        if u > v {
            let temp = u;
            u = v;
            v = temp;
        }
        v -= u;
        if v == 0 {
            break;
        }
    }
    u << shift
}

impl Gcd for usize {
    #[inline]
    fn gcd(self, other: usize) -> usize {
        self.gcd_binary(other)
    }
    #[inline]
    fn gcd_binary(self, v: usize) -> usize {
        binary_usize(self, v)
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let str1 = "QwerQwer".to_string();
        let str2 = "QwerQwerQwer".to_string();
        let result = Solution::gcd_of_strings(str1, str2);
        let expected = "Qwer".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn str2_is_prime() {
        let str1 = "QwerQwer".to_string();
        let str2 = "QwerQwerQ".to_string();
        let result = Solution::gcd_of_strings(str1, str2);
        let expected = "".to_string();
        assert_eq!(result, expected);
    }
}
