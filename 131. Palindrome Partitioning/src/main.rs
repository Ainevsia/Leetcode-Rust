fn main() {
    assert_eq!(Solution::partition(String::from("aab")), vec![
        vec!["aa","b"],
        vec!["a","a","b"]
    ]);
}

struct Solution {}

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let s: Vec<char> = s.chars().collect();
        Self::part(&s, s.len())
    }

    pub fn part(s: & Vec<char>, end: usize) -> Vec<Vec<String>> {
        if end == 0 { return vec![vec![]] }
        if end == 1 { return vec![vec![s[0].to_string()]] }
        let mut res = vec![];
        for i in 0..end {
            if Self::is_palindrome(s, i, end) {
                let mut x = Self::part(s, i);
                let palin: String = s[i..end].iter().collect();
                for i in x.iter_mut() {
                    i.push(palin.clone());
                }
                res.append(&mut x);
            }
        }
        res
    }

    pub fn is_palindrome(s: & Vec<char>, mut start: usize, mut end: usize) -> bool {
        if start + 1 == end { return true }
        end -= 1;
        while start < end {
            if s[start] != s[end] { return false }
            start += 1;
            end -= 1;
        }
        true
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn is_palindrome() {
        assert_eq!(Solution::is_palindrome(&vec!['1', '2', '1'], 0, 3), true);
        assert_eq!(Solution::is_palindrome(&vec!['1', '2', '2', '1'], 0, 4), true);
    }

    #[test]
    fn basic() {
        assert_eq!(Solution::partition(String::from("aab")), vec![
            vec!["aa","b"],
            vec!["a","a","b"]
        ]);
    }
}