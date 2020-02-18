fn main() {
    assert_eq!(Solution::is_number(String::from("0")), true);
}

struct Solution {}

impl Solution {
    pub fn is_number(s: String) -> bool {
        let s = s.trim();
        if s.chars().any(|x| !x.is_digit(10) && !['e', '+', '-', '.'].contains(&x)) {
            return false
        }
        if s.contains('e') {
            let v: Vec<&str> = s.split('e').collect();
            if v.len() != 2 { return false }
            return Self::str_is_number(v[0]) && Self::str_is_number(v[1]) &&
                v[1].chars().all(|x| !['e', '.'].contains(&x))
        }
        Self::str_is_number(s)
    }

    pub fn str_is_number(s: &str) -> bool {
        if s.len() == 0 { return false }
        let mut sg = '1';
        let (sgn, dot) = s.chars().fold((0, 0), |mut sum, x| {
            if ['+', '-'].contains(&x) { sum.0 += 1; sg = x }
            if x == '.' { sum.1 += 1 }
            sum
        });
        if sgn > 1 || dot > 1 { return false }
        if sgn == 1 {
            if s.find(sg).unwrap() != 0 || s.len() == 1 { return false }
            return Self::str_is_number(&s[1..])
        }
        if dot == 1 && s.find('.').unwrap() == 0 { return s.len() > 1 }
        
        true
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::is_number(String::from("0")), true);
        assert_eq!(Solution::is_number(String::from("0.1")), true);
        assert_eq!(Solution::is_number(String::from("2e10")), true);
        assert_eq!(Solution::is_number(String::from(" -90e3   ")), true);
        assert_eq!(Solution::is_number(String::from(" 6e-1")), true);
        assert_eq!(Solution::is_number(String::from("53.5e93")), true);
        assert_eq!(Solution::is_number(String::from(" 1e")), false);
        assert_eq!(Solution::is_number(String::from("e3")), false);
        assert_eq!(Solution::is_number(String::from("abc")), false);
        assert_eq!(Solution::is_number(String::from("1 a")), false);
        assert_eq!(Solution::is_number(String::from(" 99e2.5 ")), false);
        assert_eq!(Solution::is_number(String::from(" +-3")), false);
        assert_eq!(Solution::is_number(String::from("95a54e53")), false);
        assert_eq!(Solution::is_number(String::from(" --6 ")), false);
        assert_eq!(Solution::is_number(String::from(" ... ")), false);
        assert_eq!(Solution::is_number(String::from(" . ")), false);
        assert_eq!(Solution::is_number(String::from(" 5+1 ")), false);
        assert_eq!(Solution::is_number(String::from(" .8+ ")), false);
        assert_eq!(Solution::is_number(String::from(" 4e+ ")), false);
    }

    #[test]
    fn fail() {
        assert_eq!(Solution::is_number(String::from(" -. ")), false);
    }
}