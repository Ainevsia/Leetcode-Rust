fn main() {
    assert_eq!(Solution::word_pattern("abba".to_owned(), "dog cat cat dog".to_owned()), true);
}

struct Solution {}

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let wordlst = s.split(' ').collect::<Vec<&str>>();
        if wordlst.len() != pattern.len() { return false }

        let mut m : [Option<&str>; 26] = [None; 26];
        for (i, c) in pattern.bytes().enumerate() {
            let idx = (c - b'a') as usize;
            if let Some(v) = m[idx] {
                // already bind a key, check the value
                if v != wordlst[i] { return false }
            } else {
                // not show up, first check if duplicate
                for j in 0..26 {
                    if j == i { continue }
                    if let Some(v) = m[j] {
                        if v == wordlst[i] { return false }
                    }
                }
                // next bind the pattern
                m[idx] = Some(wordlst[i]);
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::word_pattern("abba".to_owned(), "dog cat cat dog".to_owned()), true);
        assert_eq!(Solution::word_pattern("abba".to_owned(), "dog cat cat fish".to_owned()), false);
        assert_eq!(Solution::word_pattern("aaaa".to_owned(), "dog cat cat dog".to_owned()), false);
        assert_eq!(Solution::word_pattern("abba".to_owned(), "dog dog dog dog".to_owned()), false);
    }
}