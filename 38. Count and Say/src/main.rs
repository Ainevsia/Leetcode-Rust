fn main() {
    let x = Solution::count_and_say(6);
    println!("x = {:#?}", x);
}

struct Solution {}

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if n == 1 { return "1".to_string() }
        let s: Vec<u8> = Self::count_and_say(n - 1).bytes().collect();
        let mut ret = "".to_string();
        let mut number = s[0];
        let mut cnt = 1;
        let mut i = 1;
        while i <= s.len() {
            if i >= s.len() {
                ret.push_str(format!("{}", cnt).as_str());
                ret.push(number as char);
                break
            } else if s[i] != number {
                ret.push_str(format!("{}", cnt).as_str());
                ret.push(number as char);
                number = s[i];
                cnt = 1;
            } else {
                cnt += 1;
            }
            i += 1;
        }
        ret
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::count_and_say(1), "1".to_string());
        assert_eq!(Solution::count_and_say(2), "11".to_string());
        assert_eq!(Solution::count_and_say(3), "21".to_string());
        assert_eq!(Solution::count_and_say(4), "1211".to_string());
        assert_eq!(Solution::count_and_say(5), "111221".to_string());
        assert_eq!(Solution::count_and_say(6), "312211".to_string());
        assert_eq!(Solution::count_and_say(7), "13112221".to_string());
        assert_eq!(Solution::count_and_say(8), "1113213211".to_string());
        assert_eq!(Solution::count_and_say(9), "31131211131221".to_string());
        assert_eq!(Solution::count_and_say(10), "13211311123113112211".to_string());        
    }
}