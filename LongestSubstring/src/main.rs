use std::collections::HashMap;

fn main() {}

pub struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        assert_eq!(Solution::length_of_longest_substring(String::from("sdf")), 3);
        assert_eq!(Solution::length_of_longest_substring(String::from("qwezxcvasdfqwe")), 11);
    }
}