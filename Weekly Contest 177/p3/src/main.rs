fn main() {
    assert_eq!(Solution::closest_divisors(8), [3,3]);
}

struct Solution {}

impl Solution {
    pub fn closest_divisors(num: i32) -> Vec<i32> {
        let n1 = num + 1;
        let mut d = (n1 as f64).sqrt().floor() as i32;
        while n1 % d != 0 {
            d -= 1;
        }
        let dc = n1 / d;
        let diff = dc - d;
        let n1 = num + 2;
        let mut dd = (n1 as f64).sqrt().floor() as i32;
        while n1 % dd != 0 {
            dd -= 1;
        }
        let dcc = n1 / dd;
        if dcc - dd < diff { vec![dd,dcc] } else { vec![d,dc] }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::closest_divisors(8), [3,3]);
        assert_eq!(Solution::closest_divisors(8), [3,3]);
        assert_eq!(Solution::closest_divisors(999), [40,25]);
    }
}