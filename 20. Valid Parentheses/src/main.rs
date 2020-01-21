fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut vec = vec![];
        for c in s.chars() {
            if c == '(' || c == '[' || c == '{' {
                vec.push(c);
            } else if c == ')' || c == ']' || c == '}' {
                let o = vec.pop();
                let out;
                if let None = o { return false }
                else { out = o.unwrap() }
                if  (out == '(' && c == ')') ||
                    (out == '[' && c == ']') ||
                    (out == '{' && c == '}') {
                    continue
                } else { return false }
            }
        }
        if vec.len() > 0 { false }
        else { true }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::is_valid("([)]".to_string()), false);
        assert_eq!(Solution::is_valid("{[]}".to_string()), true);
        assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
        assert_eq!(Solution::is_valid("".to_string()), true);
        assert_eq!(Solution::is_valid("[".to_string()), false);


    }
}