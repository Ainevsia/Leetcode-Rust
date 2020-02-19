fn main() {
    assert_eq!(Solution::num_decodings(String::from("*")), 9);
}

struct Solution {}

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let modu: usize = 10usize.pow(9) + 7;
        let s: Vec<u8> = s.bytes().map(|x| if x == b'*' { x } else { x - b'0' }).collect();
        let length = s.len() + 1;
        let mut dp = vec![(0, 0); length];
        dp[length - 1].0 = 1;
        dp[length - 2].0 = match s[length - 2] {
            b'*' => 9,
            0 => 0,
            _ => 1,
        };
        for i in (0..length-2).rev() {
            match s[i] {
                0 => {
                    dp[i].0 = 0;
                    dp[i].1 = 0;
                }
                b'*' => {
                    dp[i].0 = 9 * (dp[i+1].0 + dp[i+1].1);
                    dp[i].1 = if s[i+1] == b'*' { 15 } else {
                        if s[i+1] <= 6 { 2 } else { 1 }
                    } * (dp[i+2].0 + dp[i+2].1);
                }
                x => {
                    dp[i].0 = dp[i+1].0 + dp[i+1].1;
                    dp[i].1 = if s[i+1] == b'*' {
                        if x == 1 { 9 }
                        else if x == 2 { 6 }
                        else { 0 }
                    } else {
                        if s[i] * 10 + s[i+1] <= 26 { 1 } else { 0 }
                    } * (dp[i+2].0 + dp[i+2].1);
                }
            }
            dp[i].0 %= modu;
            dp[i].1 %= modu;
        }
        println!("modu = {:#?}", modu);
        println!("s = {:#?}, dp = {:#?}", s, dp);
        ((dp[0].0 + dp[0].1) % modu) as i32
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::num_decodings(String::from("*")), 9);
        assert_eq!(Solution::num_decodings(String::from("1*")), 18);
    }

    #[test]
    fn fail() {
        assert_eq!(Solution::num_decodings(String::from("**")), 96);
        assert_eq!(Solution::num_decodings(String::from("**********1111111111")), 133236775);
        // 1000000007
        // 1320548177
        //  320548170
    }
}