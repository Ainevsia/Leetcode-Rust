fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn group_anagrams_old(strs: Vec<String>) -> Vec<Vec<String>> {
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

    /// an ugly O(n^2) algo
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut strs_sorted = vec![];
        for s in strs.iter() {
            let mut v = s.bytes().collect::<Vec<u8>>();
            v.sort();
            let v = String::from_utf8(v).unwrap();
            strs_sorted.push(v);
        }
        let mut res: Vec<Vec<String>> = vec![];
        let mut idx: Vec<usize> = vec![];
        for i in 0..strs.len() {
            let mut found = false;
            for j in 0..res.len() {
                if strs_sorted[i] == strs_sorted[idx[j]] {
                    res[j].push(strs[i].clone());
                    found = true;
                    break;
                }
            }
            if ! found {
                res.push(vec![strs[i].clone()]);
                idx.push(i);
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