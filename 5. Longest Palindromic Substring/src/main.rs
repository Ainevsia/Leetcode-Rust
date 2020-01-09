fn main() {
    println!("Hello, world!");
}

pub struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        
        String::from("bab")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::longest_palindrome(String::from("babad")), "bab");
        assert_eq!(Solution::longest_palindrome(String::from("cbbd")), "bb");
    }
}