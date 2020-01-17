fn main() {
    println!("Hello, world!");
    Solution::longest_common_prefix(vec![String::from("s: &str")]);
}

struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut ret = "".to_string();
        let mut cnt = 0;
        let mut found = false;
        if strs.len() == 0 || strs[0].len() == 0 {
            return ret
        }
        loop {
            ret.push_str(&strs[0][cnt..cnt + 1]);
            for i in 0..strs.len() {
                if strs[i].len() < cnt + 1 || strs[i][0..cnt + 1] != ret {
                    found = true;
                    break;
                }
            }
            match found {
                true => break ret[0..cnt].to_string(),
                false => if cnt + 1 == strs[0].len() {
                    break ret
                }
            }
            cnt += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::longest_common_prefix(
            vec![String::from("s: &str"), 
            String::from("s: &str")]), 
            String::from("s: &str"));
        assert_eq!(Solution::longest_common_prefix(
            vec![String::from("flower"), 
            String::from("flow"),
            String::from("flight")]), 
            String::from("fl"));
        assert_eq!(Solution::longest_common_prefix(
            vec![String::from("dog"), 
            String::from("racecar"),
            String::from("flight")]), 
            String::from(""));
    }
}
