use std::collections::HashMap;
use std::collections::HashSet;
        

fn main() {
    Solution::length_of_longest_substring(String::from("tmmzuxt"));
}

pub struct Solution {}

impl Solution {
    pub fn length_of_longest_substring_bad(s: String) -> i32 {
        let len = s.len();
        let mut max = 1;
        for i in 0..len {
            let mut cnt = 1;
            let c = s.chars().nth(i).unwrap();
            let mut map = HashMap::new();
            map.insert(c, 1);
            for j in i+1..len {
                let endc = s.chars().nth(j).unwrap();
                if map.contains_key(&endc) {
                    break;
                } else {
                    map.insert(endc, 1);
                    cnt += 1;
                }
            }
            if cnt > max {
                max = cnt;
            }
        }
        max
    }

    pub fn length_of_longest_substring_ori(s: String) -> i32 {
        let mut check = Vec::with_capacity(128);
        for _i in 0..128 {
            check.push(0);
        }
        let mut max_len = 0;
        let mut len: i32 = 0;
        // let cidx = 0;
        let mut string = Vec::new();
        for c in s.chars() {
            string.push(c);
        }
        for cidx in 0..string.len() {
            let idx = (string[cidx] as u32 - ' ' as u32) as usize;
            if check[idx] == 0 {
                check[idx] = 1;
                len += 1;
                // println!("{} {} {} append", c, len, max_len);
            } else {
                if len > max_len {
                    max_len = len;
                }
                // find the repeated character
                while string[cidx - len as usize] != string[cidx] {
                    // println!("{} comparaing {}", c, s.chars().nth(cidx - len as usize).unwrap());
                    check[(string[cidx - len as usize] as u32 - ' ' as u32) as usize] = 0;
                    len -= 1;
                }
                check[idx] = 1;
                // println!("{} {} {}", string[cidx], len, max_len);
            }
        }
        if len > max_len {
            len
        } else {
            max_len
        }
    }

    pub fn length_of_longest_substring(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let mut map: HashSet<char> = HashSet::new();
        let (mut start, mut end, mut res) = (0, 0, 0);
        while end < s.len() {
            let mut dup = false;
            if map.contains(&s[end]) { dup = true }
            else { map.insert(s[end]); }
            end += 1;
            while dup {
                if s[start] == s[end - 1] { start += 1; break }
                map.remove(&s[start]);
                start += 1;
            }
            res = usize::max(end - start, res);
        }
        res as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        assert_eq!(Solution::length_of_longest_substring(String::from("sdf")), 3);
        assert_eq!(Solution::length_of_longest_substring(String::from("qwezxcvasdfqwe")), 11);
        assert_eq!(Solution::length_of_longest_substring(String::from("pwwkew")), 3);
        assert_eq!(Solution::length_of_longest_substring(String::from("dvdf")), 3);
        assert_eq!(Solution::length_of_longest_substring(String::from("tmmzuxt")), 5);
    }
}