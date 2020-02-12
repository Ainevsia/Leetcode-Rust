fn main() {
    Solution::backspace_compare(String::from("ab#c"), String::from("aa#c"));
}

struct Solution {}

impl Solution {
    /// two array walk through
    pub fn backspace_compare(s: String, t: String) -> bool {
        let mut s: Vec<char> = s.chars().collect();
        s.insert(0, ' ');
        let mut t: Vec<char> = t.chars().collect();
        t.insert(0, ' ');
        let (mut i, mut j) = (s.len() - 1, t.len() - 1);
        let (mut iskip, mut jskip) = (0, 0);
        loop {
            while s[i] == '#' || iskip > 0 || s[i] == ' ' {
                if s[i] == '#' { iskip += 1 }
                else if s[i] == ' ' { break }
                else if iskip > 0 { iskip -= 1 }
                i -= 1;
            }
            while t[j] == '#' || jskip > 0 {
                if t[j] == '#' { jskip += 1 }
                else if t[j] == ' ' { break }
                else if jskip > 0 { jskip -= 1 }
                j -= 1;
            }
            if s[i] != t[j] { break false }
            if i == 0 && j == 0 { break true }
            i -= 1;
            j -= 1;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::backspace_compare(String::from("ab#c"), String::from("aa#c")), true);
        assert_eq!(Solution::backspace_compare(String::from("ab##"), String::from("a#c#")), true);
        assert_eq!(Solution::backspace_compare(String::from("a##c"), String::from("#a#c")), true);
        assert_eq!(Solution::backspace_compare(String::from("a#c"), String::from("a#b")), false);
        assert_eq!(Solution::backspace_compare(String::from("#"), String::from("a#b#####")), true);
        assert_eq!(Solution::backspace_compare(String::from("nzp#o#g"), String::from("b#nzp#o#g")), true);


    }
}