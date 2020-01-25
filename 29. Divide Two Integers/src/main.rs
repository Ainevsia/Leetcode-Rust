fn main() {
    println!("{}", i32::max_value());
    println!("{}", i32::min_value());
    println!("{}", i32::abs(i32::min_value()));
    Solution::divide(10, 3);
}

struct Solution {}

impl Solution {
    pub fn divide(mut dividend: i32, divisor: i32) -> i32 {
        if dividend == i32::min_value() && divisor == i32::min_value()
            { return 1 }
        if divisor == i32::min_value() { return 0 }
        if dividend == i32::min_value() && i32::abs(divisor) == 1
            { return if divisor == 1 { dividend } else { i32::max_value() } }
        // now divisor cannot be i32::min_value()
        // but dividend can be i32::min_value()
        let mut ret = 0;
        let mut negate = false;
        if  dividend > 0 && divisor < 0 ||
            dividend < 0 && divisor > 0
        { negate = true }
        if dividend == i32::min_value() {
            dividend += i32::abs(divisor);
            ret += 1;
        }
        let mut dividend = i32::abs(dividend);
        let divisor = i32::abs(divisor);
        // now both dividend & divisor cannot be i32::min_value()
        if  dividend < divisor { return if negate { -ret } else { ret } }            
        if divisor == 1 { return if negate { -dividend } else { dividend } }
        while dividend >= divisor {
            dividend -= divisor;
            ret += 1;
        }
        if negate { -ret } else { ret }
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