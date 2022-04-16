fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    /// 回文数p可表示为p=(10^n-i)*(10^n-j) = (10^n-i-j)*10^n + i*j
    /// 令 upper = 10^n-i-j, lower = i*j, 两者形成回文的关系
    /// 只要使得i+j尽可能小，回文数p便可以尽可能大，枚举之
    pub fn largest_palindrome(n: i32) -> i32 {
        if n == 1 { return 9 }
        let top = u64::pow(10, n as u32);
        let mut sum = 1;
        loop {
            let upper = top - sum;
            let lower = upper.to_string().chars().rev().collect::<String>().parse().unwrap();
            let mut i = sum >> 1;
            let mut j = sum - i;
            loop {
                if i * j < lower { break }
                if i * j == lower {
                    return ((upper * top + lower) % 1337) as i32
                }
                i -= 1;
                j += 1;
            }
            sum += 1
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    use crate::Solution;

    #[test]
    fn basic() {
        assert_eq!(Solution::largest_palindrome(1),9);
        assert_eq!(Solution::largest_palindrome(2),987);
        assert_eq!(Solution::largest_palindrome(3),123);
        assert_eq!(Solution::largest_palindrome(4),597);
        assert_eq!(Solution::largest_palindrome(5),677);
        assert_eq!(Solution::largest_palindrome(6),1218);
        assert_eq!(Solution::largest_palindrome(7),877);
        assert_eq!(Solution::largest_palindrome(8),475);
    }
}