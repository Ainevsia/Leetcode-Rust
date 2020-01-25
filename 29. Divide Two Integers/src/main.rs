fn main() {
    println!("{}", i32::max_value());
    println!("{}", i32::min_value());
    println!("{}", i32::abs(i32::min_value()));
    Solution::divide(10, 3);
}

struct Solution {}

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        // deal with the only overflow case
        if dividend == i32::min_value() && divisor == -1
            { return i32::max_value() }
        let mut dvd = i64::abs(dividend as i64);
        let dvs = i64::abs(divisor as i64);
        let sign: i64 = if (dividend > 0) ^ (divisor > 0) { -1 } else { 1 };
        let mut ans = 0;
        while dvd >= dvs {
            let mut tmp = dvs;
            let mut shift: i64 = 1;
            while dvd >= (tmp << 1) {
                tmp <<= 1;
                shift <<= 1;
            }
            dvd -= tmp;
            ans += shift;
        }
        (sign * ans) as i32
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::divide(10, 3), 3);
        assert_eq!(Solution::divide(7, -3), -2);
        assert_eq!(Solution::divide(7, -7), -1);
        assert_eq!(Solution::divide(7, -8), 0);
        assert_eq!(Solution::divide(-7, 3), -2);
        assert_eq!(Solution::divide(-7, 33), 0);
        assert_eq!(Solution::divide(10, 3), 3);
    }

    #[test]
    fn edge() {
        assert_eq!(Solution::divide(i32::max_value(), i32::max_value()), 1);
        assert_eq!(Solution::divide(i32::min_value(), i32::min_value()), 1);
        assert_eq!(Solution::divide(i32::max_value(), i32::min_value()), 0);
        assert_eq!(Solution::divide(i32::min_value(), i32::max_value()), -1);
        assert_eq!(Solution::divide(i32::max_value(), 1), i32::max_value());
        assert_eq!(Solution::divide(i32::min_value(), 1), i32::min_value());
        assert_eq!(Solution::divide(i32::max_value(), -1), -i32::max_value());
        assert_eq!(Solution::divide(i32::min_value(), -1), i32::max_value());
        assert_eq!(Solution::divide( 1, i32::max_value()), 0);
        assert_eq!(Solution::divide(-1, i32::max_value()), 0);
        assert_eq!(Solution::divide( 1, i32::min_value()), 0);
        assert_eq!(Solution::divide(-1, i32::min_value()), 0);
    }

    #[test]
    fn leetcode() {
        assert_eq!(Solution::divide(i32::min_value(), -1), i32::max_value());
        assert_eq!(Solution::divide(i32::min_value(), 1), i32::min_value());
    }
}