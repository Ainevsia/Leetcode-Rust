fn main() {
    for i in 0..20 {
        let x = Solution::is_happy(i);
        println!("i = {:#?}, x = {:#?}", i, x);
    }
}

struct Solution {}

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut slow = Self::square_sum(n);
        let mut fast = Self::square_sum(slow);
        loop {
            if fast == 1 { return true }
            if slow == fast { return false }
            slow = Self::square_sum(slow);
            fast = Self::square_sum(fast);
            fast = Self::square_sum(fast);
        }
    }

    pub fn square_sum(mut n: i32) -> i32 {
        let mut sum = 0;
        while n > 0 {
            let digit = n % 10;
            sum += digit.pow(2);
            n /= 10;
        }
        println!("n = {:#?}, sum = {:#?}", n, sum);
        sum
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::is_happy(19), true);
    }
    
    #[test]
    fn square_sum() {
        assert_eq!(Solution::square_sum(19), 82);
        assert_eq!(Solution::square_sum(82), 68);
        assert_eq!(Solution::square_sum(68), 100);
        assert_eq!(Solution::square_sum(100), 1);
    }
}
