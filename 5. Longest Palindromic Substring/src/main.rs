fn main() {
    println!("Hello, world!");

    let mut bytes = "bors".bytes();

    assert_eq!(Some(b'b'), bytes.next());
    assert_eq!(Some(b'o'), bytes.next());
    assert_eq!(Some(b'r'), bytes.next());
    assert_eq!(Some(b's'), bytes.next());

    assert_eq!(None, bytes.next());

}

pub struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let vec : Vec<u8> = s.bytes().collect();
        let len = vec.len();
        if len <= 1 {
            return s;
        }
        let mut start = 0;
        let mut end = 0;
        for s in 0..len {
            for e in s+1..len {
                if Solution::is_palindromic(&vec, s, e) && e - s > end - start {
                    end = e;
                    start = s;
                }
            }
        }
        s[start..end+1].into()
    }

    fn is_palindromic(s:&[u8], start:usize, end:usize) -> bool {
        let mut end = end;
        let mut m_start = start;
        while m_start < end {
            if s[m_start] != s[end] {
                return false;
            } else {
                m_start += 1;
                end -= 1;
            }
        }
        true
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