fn main() {
    println!("Hello, world!");
    Solution::roman_to_int(String::from("III"));
}

struct Solution {}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut ret = 0;
        let mut iter = s.bytes();
        let mut previous = iter.next();
        loop {
            let current = iter.next();
            match (previous, current) {
                (Some(b'I'), Some(b'V')) => { ret += 4; previous = iter.next() }
                (Some(b'I'), Some(b'X')) => { ret += 9; previous = iter.next() }
                (Some(b'X'), Some(b'L')) => { ret += 40; previous = iter.next() }
                (Some(b'X'), Some(b'C')) => { ret += 90; previous = iter.next() }
                (Some(b'C'), Some(b'D')) => { ret += 400; previous = iter.next() }
                (Some(b'C'), Some(b'M')) => { ret += 900; previous = iter.next() }
                (None, None) => break ret,
                (Some(c), None) => {
                    match c {
                        b'I' => ret += 1,
                        b'V' => ret += 5,
                        b'X' => ret += 10,
                        b'L' => ret += 50,
                        b'C' => ret += 100,
                        b'D' => ret += 500,
                        b'M' => ret += 1000,
                        _ => ()
                    }
                    break ret;
                }
                (Some(c), Some(r)) => {
                    match c {
                        b'I' => ret += 1,
                        b'V' => ret += 5,
                        b'X' => ret += 10,
                        b'L' => ret += 50,
                        b'C' => ret += 100,
                        b'D' => ret += 500,
                        b'M' => ret += 1000,
                        _ => ()
                    }
                    previous = Some(r);
                }
                (None, Some(_)) => ()
            }
        }
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