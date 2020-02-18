fn main() {
    assert_eq!(Solution::length_of_last_word(String::from("Hello World")), 5);
}

struct Solution {}

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let s = s.trim();
        if let Some(idx) = s.rfind(' ') {
            (s.len() - 1 - idx) as i32
        } else { if s.len() > 0 { s.len() as i32 } else { 0 } }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::length_of_last_word(String::from("Hello World")), 5);
        assert_eq!(Solution::length_of_last_word(String::from("")), 0);
        assert_eq!(Solution::length_of_last_word(String::from("Hello")), 5);
        assert_eq!(Solution::length_of_last_word(String::from("H  ")), 1);
        
        assert_eq!(Solution::length_of_last_word(String::from("  ")), 0);
    }
}