fn main() {
    Solution::longest_substring(String::from("aaabb"), 3);
}

struct Solution {}

impl Solution {
    pub fn longest_substring(s: String, k: i32) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let mut res = 0;
        for n in 1..=26 {
            res = i32::max(Self::n_unique_char(&s, k, n), res);
        }
        res
    }

    pub fn n_unique_char(s: &[char], k: i32, n: usize) -> i32 {
        use std::collections::HashMap;
        let mut map: HashMap<char, i32> = HashMap::new();
        let (mut start, mut end, mut res) = (0, 0, 0);
        while end < s.len() {
            map.entry(s[end]).and_modify(|x| *x += 1).or_insert(1);
            end += 1;
            while map.len() > n {
                map.entry(s[start]).and_modify(|x| *x -= 1);
                if map.get(&s[start]).unwrap() == &0 {
                    map.remove(&s[start]);
                }
                start += 1;
            }
            if map.iter().all(|(_, v)| *v >= k) {
                res = usize::max(end - start, res);
            }
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
        assert_eq!(Solution::longest_substring(String::from("bbaaacbd"), 3), 3);

    }
    
    #[test]
    fn basic_n() {
        assert_eq!(Solution::n_unique_char(&String::from("aaabb").chars().collect::<Vec<char>>(), 3, 1), 3);
        assert_eq!(Solution::n_unique_char(&String::from("ababbc").chars().collect::<Vec<char>>(), 2, 2), 5);
        assert_eq!(Solution::n_unique_char(&String::from("aaabbb").chars().collect::<Vec<char>>(), 3, 2), 6);
        assert_eq!(Solution::n_unique_char(&String::from("weitong").chars().collect::<Vec<char>>(), 2, 1), 0);
        assert_eq!(Solution::n_unique_char(&String::from("ababacb").chars().collect::<Vec<char>>(), 3, 3), 0);
        assert_eq!(Solution::n_unique_char(&String::from("bbaaacbd").chars().collect::<Vec<char>>(), 3, 1), 3);

    }
}