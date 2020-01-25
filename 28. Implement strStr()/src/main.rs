fn main() {
    let a = String::from("abcd");
    let b = &a[0..2];
    println!("b = {:#?}", b);
    assert_eq!(b, "ab");
    if b == "ab" {
        println!("hello");
    }
    if b != "ab" {
        println!("hello");
    }
}

struct Solution {}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.len() == 0 { return 0 }
        if needle.len() > haystack.len() { return -1 }
        if haystack[..] == needle[..] { return 0 }
        let win = needle.len();
        for i in 0..haystack.len() - win + 1 {
            if needle == haystack[i..i + win] {
                return i as i32
            }
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::str_str("aaaaa".to_string(), "bba".to_string()), -1);
        assert_eq!(Solution::str_str("hello".to_string(), "ll".to_string()), 2);
        assert_eq!(Solution::str_str("hello".to_string(), "".to_string()), 0);
        assert_eq!(Solution::str_str("a".to_string(), "a".to_string()), 0);
        assert_eq!(Solution::str_str("mississippi".to_string(), "pi".to_string()), 9);
        
    }
}