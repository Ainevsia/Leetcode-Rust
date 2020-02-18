use regex::Regex;

fn main() {
    assert_eq!(Solution::is_number(String::from("0")), true);
}

struct Solution {}

impl Solution {
    pub fn is_number_(s: String) -> bool {
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

    pub fn is_number_re(s: String) -> bool {
        // this regex works
        let re = Regex::new(r"^(\+|-)?(\d+(\.\d*)?|\.\d+)(e(\+|-)?\d+)?$").unwrap();
        println!("s = {:#?}", s);
        re.is_match(s.trim())
    }

    pub fn is_number(s: String) -> bool {
        let mut state = S1;
        for c in s.trim().chars() {
            match state {
                S1 => {
                    match c {
                        '+' | '-' => state = S2,
                        '.' => state = S3,
                        _ => if "0123456789".contains(c) { state = S4 } else { return false },
                    }
                }
                S2 => {
                    match c {
                        '.' => state = S3,
                        _ => if "0123456789".contains(c) { state = S4 } else { return false },
                    }
                }
                S3 => {
                    if "0123456789".contains(c) { state = S5 } else { return false }
                }
                S4 => {
                    match c {
                        '.' => state = S5,
                        'e' => state = S6,
                        _ => if "0123456789".contains(c) { state = S4 } else { return false },
                    }
                }
                S5 => {
                    match c {
                        'e' => state = S6,
                        _ => if "0123456789".contains(c) { state = S5 } else { return false },
                    }
                }
                S6 => {
                    match c {
                        '+' | '-' => state = S7,
                        _ => if "0123456789".contains(c) { state = S8 } else { return false },
                    }
                }
                S7 => {
                    if "0123456789".contains(c) { state = S8 } else { return false }
                }
                S8 => {
                    if "0123456789".contains(c) { state = S8 } else { return false }
                }
            }
        }
        // println!("s = {:#?}, state = {:#?}", s, state);
        if state == S4 || state == S5 || state == S8 { true } else { false }
    }
}

#[derive(PartialEq, Debug)]
enum State {S1, S2, S3, S4, S5, S6, S7, S8}

use State::{S1, S2, S3, S4, S5, S6, S7, S8};

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::is_number(String::from("0")), true);
        assert_eq!(Solution::is_number(String::from("0.1")), true);
        assert_eq!(Solution::is_number(String::from("2e10")), true);
        assert_eq!(Solution::is_number(String::from("e3")), false);
        assert_eq!(Solution::is_number(String::from("abc")), false);
        assert_eq!(Solution::is_number(String::from("1 a")), false);
        assert_eq!(Solution::is_number(String::from(" 99e2.5 ")), false);
        assert_eq!(Solution::is_number(String::from(" +-3")), false);
        assert_eq!(Solution::is_number(String::from("95a54e53")), false);
        assert_eq!(Solution::is_number(String::from(" --6 ")), false);
        assert_eq!(Solution::is_number(String::from(" .8+ ")), false);
        assert_eq!(Solution::is_number(String::from(" 4e+ ")), false);
    }

    #[test]
    fn fail() {
        assert_eq!(Solution::is_number(String::from(" -. ")), false);
        assert_eq!(Solution::is_number(String::from(" -90e3   ")), true);
        assert_eq!(Solution::is_number(String::from(" ... ")), false);
        assert_eq!(Solution::is_number(String::from(" . ")), false);
        assert_eq!(Solution::is_number(String::from(" 5+1 ")), false);
        assert_eq!(Solution::is_number(String::from(" 6e-1")), true);
        assert_eq!(Solution::is_number(String::from("53.5e93")), true);
        assert_eq!(Solution::is_number(String::from(" 1e")), false);
    }

    #[test]
    fn leetcode() {
        assert_eq!(Solution::is_number(String::from("123")), true);
        assert_eq!(Solution::is_number(String::from(" 123 ")), true);
        assert_eq!(Solution::is_number(String::from("0")), true);
        assert_eq!(Solution::is_number(String::from("0123")), true);  //Cannot agree
        assert_eq!(Solution::is_number(String::from("00")), true);  //Cannot agree
        assert_eq!(Solution::is_number(String::from("-10")), true);
        assert_eq!(Solution::is_number(String::from("-0")), true);
        assert_eq!(Solution::is_number(String::from("123.5")), true);
        assert_eq!(Solution::is_number(String::from("123.000000")), true);
        assert_eq!(Solution::is_number(String::from("-500.777")), true);
        assert_eq!(Solution::is_number(String::from("0.0000001")), true);
        assert_eq!(Solution::is_number(String::from("0.00000")), true);
        assert_eq!(Solution::is_number(String::from("0.")), true);  //Cannot be more disagree!!!
        assert_eq!(Solution::is_number(String::from("00.5")), true);  //Strongly cannot agree
        assert_eq!(Solution::is_number(String::from("123e1")), true);
        assert_eq!(Solution::is_number(String::from("1.23e10")), true);
        assert_eq!(Solution::is_number(String::from("0.5e-10")), true);
        assert_eq!(Solution::is_number(String::from("1.0e4.5")), false);
        assert_eq!(Solution::is_number(String::from("0.5e04")), true);
        assert_eq!(Solution::is_number(String::from("12 3")), false);
        assert_eq!(Solution::is_number(String::from("1a3")), false);
        assert_eq!(Solution::is_number(String::from("")), false);
        assert_eq!(Solution::is_number(String::from("   ")), false);
        assert_eq!(Solution::is_number(String::from(".1")), true); //Ok)), if you say so
        assert_eq!(Solution::is_number(String::from(".")), false);
        assert_eq!(Solution::is_number(String::from("2e0")), true);  //Really?!
        assert_eq!(Solution::is_number(String::from("+.8")), true);  
        assert_eq!(Solution::is_number(String::from(" 005047e+6")), true);  //Damn = =|||
    }
}