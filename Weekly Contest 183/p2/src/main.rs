fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let mut s: Vec<char> = s.chars().collect();
        let mut res = 0;
        while s.len() > 1 {
            res += 1;
            if s[s.len() - 1] == '0' {
                s.pop();
            } else {
                let mut i = s.len() - 1;
                while s[i] == '1' {
                    s[i] = '0';
                    if i == 0 {
                        break
                    } else {
                        i -= 1;
                    }
                }
                if i == 0 && s[0] == '0' {
                    s.insert(0, '1');
                } else {
                    s[i] = '1';
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
        assert_eq!(Solution::num_steps(String::from("1101")), 6);
        assert_eq!(Solution::num_steps(String::from("10")), 1);
        assert_eq!(Solution::num_steps(String::from("1")), 0);
    }
}