fn main() {
    assert!('K'.is_alphanumeric());
    assert!('و'.is_alphanumeric());
    let a = "123".to_owned();
    let a = a.as_bytes();
    let b = a[1];
    let res = Solution::is_palindrome("A man, a plan, a canal: Panama".to_owned());
    println!("res = {:#?}", res);
}

struct Solution {}

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s = s.as_bytes();
        let mut end = s.len() - 1;
        let mut start = 0;
        loop {
            if start >= end { break }
            while start < s.len() && ! s[start].is_ascii_alphanumeric() { start += 1; }
            while end > 0 &&  ! s[end].is_ascii_alphanumeric() { end -= 1; }
            if start >= end { break }
            if start > end { return true ; }
            let left = s[start].to_ascii_lowercase();
            let right = s[end].to_ascii_lowercase();
            // println!("left = {:#?}", left);
            // println!("right = {:#?}", right);
            if left != right { return false; }
            if start == s.len() - 1 || end == 0 { return true; }
            else { start +=1 ; end -= 1; }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::is_palindrome("A man, a plan, a canal: Panama".to_owned()),true);
        assert_eq!(Solution::is_palindrome("race a c ar".to_owned()),false);
    }
}