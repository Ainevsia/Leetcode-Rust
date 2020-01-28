fn main() {
    Solution::longest_valid_parentheses("())".to_string());
}

struct Solution {}

impl Solution {
    pub fn longest_valid_parentheses_my(s: String) -> i32 {
        let mut y = vec![0];
        let mut h = 0;
        for c in s.chars() {
            h += if c == '(' { 1 } else { -1 };
            y.push(h);
        }
        while y.len() > 1 && y[0] > y[1]
            { y.remove(0); }
        while y.len() > 1 && y[y.len() - 2] < y[y.len() - 1]
            { y.pop(); }
        if y.len() < 3 { return 0 }
        let mut x = vec![0];
        let mut i = 1;
        while i < y.len() - 1 {
            if y[i - 1] > y[i] && y[i] < y [i + 1] { x.push(i); }
            i += 1;
        }
        x.push(y.len() - 1);
        loop {
            let thresh = std::cmp::min(y[x[0]], y[x[x.len() - 1]]);
            let mut accept = true;
            i = 1;
            while i < x.len() - 1 {
                if y[x[i]] < thresh {
                    accept = false;
                    break
                }
            }
            if !accept {

            }
        }
        println!("vec = {:#?}", x);
        unimplemented!()
    }

    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut vec = vec![];
        for (i, c) in s.chars().enumerate() {
            println!("vec = {:#?}", vec);
            if c == '(' { vec.push(Term::new('(', i)); }
            else if vec.is_empty() { vec.push(Term::new(')', i)); }
            else if vec[vec.len()-1].p() == '(' { vec.pop(); }
            else { vec.push(Term::new(')', i)); }
        }
        println!("vec = {:#?}", vec);
        let mut max: i32 = 0;
        let mut high = s.len();
        if vec.is_empty() { return s.len() as i32 }
        while !vec.is_empty() {
            let i = vec.pop().unwrap().i();
            let diff = high - i - 1;
            high = i;
            if diff as i32 > max { 
                max = diff as i32;
            }
            if vec.is_empty() {
                if i as i32 > max { 
                    max = i as i32;
                }
            }
        }
        max
    }
}

#[derive(Debug)]
struct Term {
    p: char,
    idx: usize,
}

impl Term {
    pub fn new(p: char, idx: usize) -> Term {
        Term { p, idx }
    }

    pub fn p(&self) -> char {
        self.p
    }

    pub fn i(&self) -> usize {
        self.idx
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::longest_valid_parentheses("()".to_string()), 2);
        assert_eq!(Solution::longest_valid_parentheses("(()".to_string()), 2);
        assert_eq!(Solution::longest_valid_parentheses("())".to_string()), 2);

        assert_eq!(Solution::longest_valid_parentheses(")()())".to_string()), 4);

        assert_eq!(Solution::longest_valid_parentheses("((())))(((()))".to_string()), 6);

    }

    #[test]
    fn edge() {
        assert_eq!(Solution::longest_valid_parentheses("".to_string()), 0);
        assert_eq!(Solution::longest_valid_parentheses("))))".to_string()), 0);
        assert_eq!(Solution::longest_valid_parentheses(")))(((".to_string()), 0);
        assert_eq!(Solution::longest_valid_parentheses(")))((((".to_string()), 0);
        
    }
}
