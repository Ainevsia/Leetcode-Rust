fn main() {
    assert_eq!(Solution::check_valid_string(String::from("()")), true);
}

struct Solution {}

impl Solution {
    /// Time:  O(n) two pass
    /// Space: O(1)
    /// just scan the array from right to left to determine each use of *
    /// then scan the array from left to right to determine each use of *
    pub fn check_valid_string(s: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let (mut w, mut r) = (0, 0);
        for &c in s.iter().rev() {
            match c {
                '*' => w += 1,
                ')' => r += 1,
                '(' | _ => {
                    if r > 0 { r -= 1 }
                    else if w > 0 { w -= 1 }
                    else { return false }
                }
            }
        }
        if r > w { return false }
        let (mut w, mut l) = (0, 0);
        for &c in s.iter() {
            match c {
                '*' => w += 1,
                '(' => l += 1,
                ')' | _ => {
                    if l > 0 { l -= 1 }
                    else if w > 0 { w -= 1 }
                    else { return false }
                }
            }
        }
        l <= w
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::check_valid_string(String::from("()")), true);
        assert_eq!(Solution::check_valid_string(String::from("(*)")), true);
        assert_eq!(Solution::check_valid_string(String::from("(*))")), true);
        assert_eq!(Solution::check_valid_string(String::from("(*)))")), false);
        assert_eq!(Solution::check_valid_string(String::from("(*)(()")), false);
        assert_eq!(Solution::check_valid_string(String::from("(****")), true);
    }

    #[test]
    fn fail() {
        
        assert_eq!(Solution::check_valid_string(String::from(")))***")), false);
        // assert_eq!(Solution::check_valid_string(String::from("(()(()))(()()()))))((((()*()*(())())(()))((*()(*((*(*()))()(())*()()))*)*()))()()(())()(()))())))")), false);
    }
}