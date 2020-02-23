fn main() {
    assert_eq!(Solution::closest_divisors(8), [3,3]);
}

struct Solution {}

impl Solution {
    pub fn closest_divisors(num: i32) -> Vec<i32> {
        let n = num + 1;
        let mut d1 = (n as f64).sqrt().floor() as i32;
        while n % d1 != 0 { d1 -= 1; }
        let dc1 = n / d1;
        
        let n = num + 2;
        let mut d2 = (n as f64).sqrt().floor() as i32;
        while n % d2 != 0 {
            d2 -= 1;
        }
        let dc2 = n / d2;

        if dc2 - d2 < dc1 - d1 { vec![d2,dc2] } else { vec![d1,dc1] }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::closest_divisors(8), [3,3]);
        assert_eq!(Solution::closest_divisors(8), [3,3]);
        assert_eq!(Solution::closest_divisors(999), [25, 40]);
    }
}