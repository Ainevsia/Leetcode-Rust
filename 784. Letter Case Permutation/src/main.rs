fn main() {
    Solution::letter_case_permutation(String::from("a1b2"));
}

struct Solution {}

impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        let mut res: Vec<String> = vec!["".to_string()];
        for c in s.chars() {
            // println!("c = {:#?}, res = {:#?}", c, res);
            if c.is_digit(10) { res = res.into_iter().map(|mut x| {x.push(c); x}).collect() }
            else {
                for i in 0..res.len() {
                    let mut x = res[i].clone();
                    x.push(c.to_ascii_lowercase());
                    res.push(x);
                    res[i].push(c.to_ascii_uppercase());
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::letter_case_permutation(String::from("a1b2")),
        vec!["A1B2","a1B2","A1b2","a1b2"]);

        assert_eq!(Solution::letter_case_permutation(String::from("C")),
        vec!["C","c"]);
    }
}