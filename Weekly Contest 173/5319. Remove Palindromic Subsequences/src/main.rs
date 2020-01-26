fn main() {
    let mut s = String::from("α is alpha, β is beta");
    let beta_offset = s.find('β').unwrap_or(s.len());

    // Remove the range up until the β from the string
    let t: String = s.drain(..beta_offset).collect();
    assert_eq!(t, "α is alpha, ");
    assert_eq!(s, "β is beta");

    // A full range clears the string
    s.drain(..);
    assert_eq!(s, "");
    let x = Solution::remove_palindrome_sub("abb".to_string());
    println!("x = {:#?}", x);
}

struct Solution {}

impl Solution {
    pub fn remove_palindrome_sub(s: String) -> i32 {
        if s.len() == 0 { return 0 }
        if Self::is_palindromic(&s.bytes().collect::<Vec<u8>>()) {
            1
        } else {
            2
        }
    }

    fn is_palindromic(s: &[u8]) -> bool {
        let mut end = s.len() - 1;
        let mut start = 0;
        while start < end {
            if s[start] != s[end] {
                return false;
            } else {
                start += 1;
                end -= 1;
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::remove_palindrome_sub("ababa".to_string()), 1);
        assert_eq!(Solution::remove_palindrome_sub("abb".to_string()), 2);
        assert_eq!(Solution::remove_palindrome_sub("baabb".to_string()), 2);
        assert_eq!(Solution::remove_palindrome_sub("".to_string()), 0);
        assert_eq!(Solution::remove_palindrome_sub("bbaabaaa".to_string()), 2);

    }
}