fn main() {
    println!("Hello, world!");
    let ret = Solution::my_atoi(String::from("  -0012a42"));
    print!("{}", ret);
}

pub struct Solution {}

impl Solution {
    pub fn my_atoi(input: String) -> i32 {
        let vec = input.as_bytes();
        let len = vec.len();
        if len == 0 {
            return 0;
        }
        let mut num_start = 0;
        let mut sign = 1;
        for i in num_start..len {
            if vec[i] as char != ' ' {
                num_start = i;
                break;
            }
        }
        use std::collections::HashSet;
        let mut digits = HashSet::new();
        for i in ('0' as u8)..('9' as u8 + 1) {
            digits.insert(i as char);
        }
        if vec[num_start] as char == '-' {
            sign = -1;
            num_start += 1;
        } else if vec[num_start] as char == '+' {
            num_start += 1;
        }
        let mut sum = 0;
        for i in num_start..len {
            let c_char = vec[i] as char;
            if ! digits.contains(&c_char) {
                return sign * sum;
            }
            let d = vec[i] - '0' as u8;
            if sum.checked_mul(10).is_none() {
                if sign == 1 {
                    return i32::max_value();
                } else {
                    return i32::min_value();
                }
            }
            sum = sum * 10 + d as i32;
        }
        // now num_start points to the start of the number
        println!("{}", sign);
        sign * sum
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::my_atoi(String::from("42")), 42);
        assert_eq!(Solution::my_atoi(String::from("   -42")), -42);
        assert_eq!(Solution::my_atoi(String::from("4193 with words")), 4193);
        assert_eq!(Solution::my_atoi(String::from("words and 987")), 0);
        assert_eq!(Solution::my_atoi(String::from("  +4193")), 4193);
        assert_eq!(Solution::my_atoi(String::from("-91283472332")), -2147483648);
        assert_eq!(Solution::my_atoi(String::from("  -0012a42")), -12);

        assert_eq!(Solution::my_atoi(String::from("2147483648")), 2147483647);
        assert_eq!(Solution::my_atoi(String::from("   -42")), -42);
        assert_eq!(Solution::my_atoi(String::from("4193 with words")), 4193);
        assert_eq!(Solution::my_atoi(String::from("words and 987")), 0);
        assert_eq!(Solution::my_atoi(String::from("  +4193")), 4193);
        assert_eq!(Solution::my_atoi(String::from("-91283472332")), -2147483648);
        assert_eq!(Solution::my_atoi(String::from("  -0012a42")), -12);
    }
}