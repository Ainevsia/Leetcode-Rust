fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    /// do the same thing as powi(n): pub fn powi(self, n: i32) -> f64
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n == 0 { return 1f64 }
        else if n == 1 { return x }
        else if n == -1 { return 1f64 / x }
        else if n == i32::min_value() {
            return Self::my_pow(x * x, n + 1)
        } else if n < 0 {
            return Self::my_pow(1f64 / x, -n);
        } else {
            if n % 2 == 0 {
                return Self::my_pow(x * x, n / 2)
            } else {
                return x * Self::my_pow(x * x, (n - 1) / 2)
            }
        }
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn example() {
        assert_eq!(Solution::my_pow(2.00000, 10), 1024.00000);
        assert_eq!(Solution::my_pow(2.10000, 3), 9.261000000000001);
        assert_eq!(Solution::my_pow(2.00000, -2), 0.25000);
        
    }

    #[test]
    fn tle() {
        assert_eq!(Solution::my_pow(0.00001, 2147483647), 0.00001f64.powi(2147483647));

    }
}