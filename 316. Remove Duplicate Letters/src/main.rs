fn main() {
    let mut x = 0;
    for i in x..11 {

        x += 2;
        println!("x = {:#?}", x);
    }
    // assert_eq!(Solution::remove_duplicate_letters(String::from("bcabc")), String::from("abc"));
}

struct Solution {}

impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let s: Vec<char> = s.chars().collect();
        use std::collections::HashMap;
        let mut map: HashMap<char, usize> = HashMap::new();
        let mut res: Vec<char> = vec![];
        for i in 0..s.len() {
            map.entry(s[i]).and_modify(|x| *x = i ).or_insert(i);
        }
        let mut prev = 0;
        while !map.is_empty() {
            // find the smallest idx
            let (mut c, idx) = map.iter().fold((' ', s.len()), |sum, (&k, &v)| if v < sum.1 { (k, v) } else { sum });
            // check whether smaller exist before
            let mut changed = false;
            for i in prev..idx {
                if s[i] <= c && map.get(&s[i]).is_some() {
                    if s[i] < c {
                        c = s[i]; prev = i + 1; changed = true
                    } else if s[i] == c {
                        if !changed { prev = i + 1; changed = true }
                    }
                }
            }
            if c == s[idx] && !changed { prev = idx + 1 }
            map.remove(&c);
            res.push(c);
        }
        res.iter().collect()
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::remove_duplicate_letters(String::from("bcabc")), String::from("abc"));
        assert_eq!(Solution::remove_duplicate_letters(String::from("cbacdcbc")), String::from("acdb"));
        assert_eq!(Solution::remove_duplicate_letters(String::from("aaabdddba")), String::from("abd"));
        assert_eq!(Solution::remove_duplicate_letters(String::from("abacb")), String::from("abc"));
    }

    #[test]
    fn fail() {
        assert_eq!(Solution::remove_duplicate_letters(
            String::from("eywdgenmcnzhztolafcfnirfpuxmfcenlppegrcalgxjlajxmphwidqqtrqnmmbssotoywfrtylm")),
            String::from("chzafipuegjlxdqnbsotwrym"));

    }
}