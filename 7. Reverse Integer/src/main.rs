fn main() {
    println!("max i32: {}", i32::max_value());
    println!("min i32: {}", i32::min_value());
    println!("{}", -11 % 10);
    print!("return value {}", 
    Solution::reverse(1534236469));
}

struct Solution {}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut c_value = x;
        let mut sum = 0;
        let mut digit = c_value % 10;
        c_value /= 10;
        while c_value != 0 {
            sum *= 10;
            sum += digit;
            digit = c_value % 10;
            c_value /= 10;
            println!("{} {} \t\t sum {}", c_value, digit, sum);
        }
        if sum >  214748364 || sum ==  214748364 && digit >  7 
        || sum < -214748364 || sum == -214748364 && digit < -8 {
            0
        } else {
            sum * 10 + digit
        }
    }

    pub fn good_reverse(x: i32) -> i32 {
        x.signum() * x
			.abs()
			.to_string()
			.chars()
			.rev()
			.collect::<String>()
			.parse::<i32>()
            .unwrap_or(0)
            
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(0), 0);
        assert_eq!(Solution::reverse(1534236469), 0);
    }
}
