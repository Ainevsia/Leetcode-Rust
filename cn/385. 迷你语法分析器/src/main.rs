use std::vec;

fn main() {
    println!("Hello, world!");
}


#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
  Int(i32),
  List(Vec<NestedInteger>)
}

struct Solution {}

impl Solution {
    pub fn deserialize(s_: String) -> NestedInteger {
        use std::str::FromStr;
        let s = s_.as_bytes();
        if s.last().unwrap() != &b']' {
            return NestedInteger::Int(i32::from_str(&s_).unwrap())
        }
        let mut res = vec![];
        let mut idx = 1;
        while idx < s.len() - 1 {
            if s[idx] == b'[' {
                let start = idx;
                let mut cnt = 0;
                idx += 1;
                loop {
                    if s[idx] == b'[' { cnt += 1 }
                    else if s[idx] == b']' {
                        if cnt > 0 { cnt -= 1 } else { break }
                    }
                    idx += 1;
                }
                idx += 1;
                res.push(Self::deserialize(s_[start..idx].to_string()));
            } else if s[idx] == b',' {
                idx += 1;
            } else {
                let start = idx;
                while s[idx] != b',' && s[idx] != b']' { idx += 1 }
                res.push(Self::deserialize(s_[start..idx].to_string()));
                idx += 1;
            }
        }
        
        NestedInteger::List(res)
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        let target = NestedInteger::Int(324);
        assert_eq!(Solution::deserialize("324".to_string()),target);
        let target = NestedInteger::List(vec![
            NestedInteger::Int(123),
            NestedInteger::List(vec![
                NestedInteger::Int(456),
                NestedInteger::List(vec![
                    NestedInteger::Int(789)])])]);
        assert_eq!(
            Solution::deserialize("[123,[456,[789]]]".to_string()),
            target);

    }
}