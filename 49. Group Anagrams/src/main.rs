fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        if strs.len() <= 1 { return vec![strs] }
        use std::collections::HashMap;
        let mut dic: HashMap<String, Vec<String>> = HashMap::new();
        for os in strs {    // makes ownership of strs
            let mut s: Vec<char> = os.chars().collect();
            s.sort();
            let s: String = s.iter().collect();
            dic.entry(s).and_modify(|x| x.push(os.clone())).or_insert(vec![os]);
        }
        dic.into_iter().fold(vec![], |mut sum, (_, vec)| {
            sum.push(vec);
            sum
        })
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        let input = vec!["eat", "tea", "tan", "ate", "nat", "bat"];
        let input: Vec<String> = input.iter().map(|x| x.to_string()).collect();
        let output = vec![
            vec!["ate","eat","tea"],
            vec!["nat","tan"],
            vec!["bat"]
        ];
        let output: Vec<Vec<String>> = output.iter().map(|x| {
            x.iter().map(|y| {
                y.to_string()
            }).collect()
        }).collect();
        assert_eq!(Solution::group_anagrams(input), output);
    }
}