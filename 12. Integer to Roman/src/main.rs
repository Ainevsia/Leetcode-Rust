fn main() {
    println!("Hello, world!");
    Solution::int_to_roman(12);
}

struct Solution {}

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut num = num;
        let mut ret = String::from("");
        while num > 0 {
            if num >= 1000 {
                ret.push_str("M");
                num -= 1000;
            } else if num >= 900 {
                ret.push_str("CM");
                num -= 900;
            } else if num >= 500 {
                ret.push_str("D");
                num -= 500;
            } else if num >= 400 {
                ret.push_str("CD");
                num -= 400;
            } else if num >= 100 {
                ret.push_str("C");
                num -= 100;
            } else if num >= 90 {
                ret.push_str("XC");
                num -= 90;
            } else if num >= 50 {
                ret.push_str("L");
                num -= 50;
            } else if num >= 40 {
                ret.push_str("XL");
                num -= 40;
            } else if num >= 10 {
                ret.push_str("X");
                num -= 10;
            } else if num >= 9 {
                ret.push_str("IX");
                num -= 9;
            } else if num >= 5 {
                ret.push_str("V");
                num -= 5;
            } else if num >= 4 {
                ret.push_str("IV");
                num -= 4;
            } else {
                ret.push_str("I");
                num -= 1;
            }
        }
        ret
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::int_to_roman(3), String::from("III"));
        assert_eq!(Solution::int_to_roman(4), String::from("IV"));
        assert_eq!(Solution::int_to_roman(9), String::from("IX"));
        assert_eq!(Solution::int_to_roman(58), String::from("LVIII"));
        assert_eq!(Solution::int_to_roman(499), String::from("CDXCIX"));
        assert_eq!(Solution::int_to_roman(1994), String::from("MCMXCIV"));
        assert_eq!(Solution::int_to_roman(3999), String::from("MMMCMXCIX"));
    }
}