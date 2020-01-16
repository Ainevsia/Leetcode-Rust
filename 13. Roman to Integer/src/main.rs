fn main() {
    println!("Hello, world!");
    Solution::roman_to_int(String::from("III"));
}

struct Solution {}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut ret = 0;
        let mut iter = s.bytes();
        let len = s.len();
        let mut i = 0;
        while i < len {
            let first = iter.nth(i).unwrap();
            let mut second;
            if first == b'I' {
                if i + 1 != len {
                    second = iter.nth(i + 1).unwrap();
                    if second == b'V' {
                        ret += 4;
                        i += 2; 
                    } else if second == b'X' {
                        ret += 9;
                        i += 2; 
                    }
                }
            }
        }
        ret
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::roman_to_int(String::from("III")), 3);
        assert_eq!(Solution::roman_to_int(String::from("IV")), 4);
        assert_eq!(Solution::roman_to_int(String::from("IX")), 9);
        assert_eq!(Solution::roman_to_int(String::from("LVIII")), 58);
        assert_eq!(Solution::roman_to_int(String::from("MCMXCIV")), 1994);
    }
}