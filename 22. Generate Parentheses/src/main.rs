fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = vec![];
        let mut substring = "".to_string();
        Self::gen(&mut result, n, n, &mut substring);
        result
    }

    fn gen(res: &mut Vec<String>, l: i32, r: i32, front: &mut String) {
        if l == 0 && r == 0 {
            res.push(front.to_string());
            return
        }
        if l == r {
            front.push('(');
            Self::gen(res, l - 1, r, front);
            front.pop();
        } else if l < r {
            if l != 0 {
                front.push('(');
                Self::gen(res, l - 1, r, front);
                front.pop();
            }
            front.push(')');
            Self::gen(res, l, r - 1, front);
            front.pop();
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(
            Solution::generate_parenthesis(3),
            vec![
                "((()))",
                "(()())",
                "(())()",
                "()(())",
                "()()()"
            ].iter().map(|x| x.to_string()).collect::<Vec<String>>()
        );
    }
}
