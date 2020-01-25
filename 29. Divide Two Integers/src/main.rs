fn main() {
    println!("{}", i32::max_value());
    println!("{}", i32::min_value());
    println!("{}", i32::abs(i32::min_value()));
}

struct Solution {}

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if  dividend > 0 && divisor > 0 && dividend < divisor ||
            dividend < 0 && divisor < 0 && dividend > divisor ||
            dividend > 0 && divisor < 0 && divisor == i32::min_value() ||
            dividend > 0 && divisor < 0 && -divisor > dividend ||
            dividend < 0 && divisor > 0 && -divisor < dividend ||
            dividend == 0
        { return 0 }
        let mut negate = false;
        if  dividend > 0 && divisor < 0 ||
            dividend < 0 && divisor > 0
        { negate = true }
        let mut ret = 0;
        let mut dividend = i32::abs(dividend);
        let divisor = i32::abs(divisor);
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
}