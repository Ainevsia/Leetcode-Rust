fn main() {
    println!("Hello, world!");
    Solution::is_match(String::from(""), String::from("aa"));
}

struct Solution {}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let (ls, lp) = (s.len(), p.len());
        if ls == 0 || lp == 0 {
            return false;
        }
        let b = p.chars();
        let mut dp = vec![vec![false; lp]; ls];
        dp[0][0] = true;
        for j in 1..lp {
            b.take(j);
        }
        let mut chars = "gravy train".chars().fuse();

        while let Some(c) = chars.next() {
            if c == 'x' {
                chars.next(); // Skip the next one
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
        assert_eq!(Solution::is_match(String::from("aa"), String::from("a")), false);
        assert_eq!(Solution::is_match(String::from("aa"), String::from("a*")), true);
        assert_eq!(Solution::is_match(String::from("ab"), String::from(".*")), true);
        assert_eq!(Solution::is_match(String::from("ab"), String::from("c*a*b")), true);
        assert_eq!(Solution::is_match(String::from("mississippi"), String::from("mis*is*p*.")), false);
        assert_eq!(Solution::is_match(String::from("mississippi"), String::from("mis*is*.p*.")), true);
    }
}