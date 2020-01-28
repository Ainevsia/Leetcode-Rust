fn main() {
    // Solution::longest_valid_parentheses("((()()()))".to_string());
    Solution::longest_valid_parentheses("".to_string());

}

struct Solution {}

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut vec = vec![0];
        let mut n = 0;
        for c in s.chars() {
            n += if c == '(' { 1 } else { -1 };
            vec.push(n);
        }
        while vec.len() > 1 && vec[0] > vec[1]
            { vec.remove(0); }
        while vec.len() > 1 && vec[vec.len() - 2] < vec[vec.len() - 1]
            { vec.pop(); }
        if vec.len() < 3 { return 0 }
        
        println!("vec = {:#?}", vec);
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    // #[test]
    fn basic() {
        assert_eq!(Solution::longest_valid_parentheses("(()".to_string()), 2);
        assert_eq!(Solution::longest_valid_parentheses(")()())".to_string()), 4);
    }

    #[test]
    fn edge() {
        assert_eq!(Solution::longest_valid_parentheses("".to_string()), 0);
        assert_eq!(Solution::longest_valid_parentheses("))))".to_string()), 0);
        assert_eq!(Solution::longest_valid_parentheses(")))(((".to_string()), 0);
        assert_eq!(Solution::longest_valid_parentheses(")))((((".to_string()), 0);

        
    }
}
