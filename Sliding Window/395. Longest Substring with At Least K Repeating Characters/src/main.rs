fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn longest_substring(s: String, k: i32) -> i32 {
        let s: Vec<char> = s.chars().collect();
        use std::collections::HashMap;
        let mut map: HashMap<char, i32> = HashMap::new();
        let (mut start, mut end, mut res) = (0, 0, 0);
        for &c in s.iter() {
            map.entry(c).and_modify(|x| *x += 1).or_insert(1);
        }
        let (less, _): (HashMap<char, i32>, HashMap<char, i32>) = map.iter().partition(|(_, v)| **v < k);
        if less.is_empty() { return s.len() as i32 }
        while end < s.len() {
            if less.get(&s[end]).is_some() {
                // split here
                res = usize::max(end - start, res);
                start = end + 1;
            }
            end += 1; 
        }
        res as i32
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::longest_substring(String::from("aaabb"), 3), 3);
        assert_eq!(Solution::longest_substring(String::from("ababbc"), 2), 5);
        assert_eq!(Solution::longest_substring(String::from("aaabbb"), 3), 6);
        assert_eq!(Solution::longest_substring(String::from("weitong"), 2), 0);
        assert_eq!(Solution::longest_substring(String::from("ababacb"), 3), 0);

    }
    
}