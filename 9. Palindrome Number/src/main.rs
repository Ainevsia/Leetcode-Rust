fn main() {
    println!("Hello, world!");
    Solution::is_palindrome(123);
    println!("x.to_string() = {:?}", 1.to_string().chars().rev());
    println!("x.to_string() = {:?}", (-2).to_string().chars().rev());
    println!("x.to_string() = {:?}", 1000.to_string().chars().rev());
    let a = String::from("sd");
    let a:String = a.chars().rev().collect();
}

pub struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let x = x.to_string();
        let r : String = x.chars().rev().collect();
        match r == x {
            true => true,
            _ => false
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::is_palindrome(123), false);
        assert_eq!(Solution::is_palindrome(10), false);
        assert_eq!(Solution::is_palindrome(0), true);
        assert_eq!(Solution::is_palindrome(-1), false);
    }
}