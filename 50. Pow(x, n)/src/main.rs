fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    /// do the same thing as powi(n): pub fn powi(self, n: i32) -> f64
    pub fn my_pow(x: f64, mut n: i32) -> f64 {
        if n == 0 { return 1f64 }
        let mut ret = 1f64;
        if n > 0 {
            while n > 0 {
                ret *= x;
                n -= 1;
            }
            return ret;
        }
        if n < 0 {
            while n < 0 {
                ret *= x;
                n += 1;
            }
            return 1f64 / ret;
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