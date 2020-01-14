fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::is_match(String::from("aa"), String::from("a")), false);
        assert_eq!(Solution::is_match(String::from("aa"), String::from("a*")), true);
        assert_eq!(Solution::is_match(String::from("ab"), String::from(".*")), true);
        assert_eq!(Solution::is_match(String::from("ab"), String::from("c*a*b")), true);
        assert_eq!(Solution::is_match(String::from("mississippi"), String::from("mis*is*p*.")), false);
        assert_eq!(Solution::is_match(String::from("mississippi"), String::from("mis*is*.p*.")), true);
    }
}