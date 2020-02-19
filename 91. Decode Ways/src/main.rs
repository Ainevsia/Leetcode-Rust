fn main() {
    assert_eq!(Solution::num_decodings(String::from("12")), 2);
}

struct Solution {}

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let s: Vec<u8> = s.bytes().map(|x| x - b'0').collect();
        let length = s.len() + 1;
        let mut dp = vec![(0, 0); length];
        dp[length - 1].0 = 1;
        dp[length - 2].0 = if s[length - 2] == 0 { 0 } else { 1 };
        for i in (0..length-2).rev() {
            dp[i].0 = if s[i] == 0 { 0 } else { dp[i+1].0 + dp[i+1].1 };
            if s[i] != 0 && s[i] * 10 + s[i+1] <= 26 { dp[i].1 = dp[i+2].0 + dp[i+2].1 }
        }
        dp[0].0 + dp[0].1
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::num_decodings(String::from("12")), 2);
        assert_eq!(Solution::num_decodings(String::from("226")), 3);
        assert_eq!(Solution::num_decodings(String::from("1")), 1);
        assert_eq!(Solution::num_decodings(String::from("0")), 0);
        assert_eq!(Solution::num_decodings(String::from("123062305230042")), 0);
        assert_eq!(Solution::num_decodings(String::from("120306230523042")), 0);
    }

    #[test]
    fn fail() {
        assert_eq!(Solution::num_decodings(String::from("1010")), 1);
    }
}