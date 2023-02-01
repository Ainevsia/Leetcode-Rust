fn main() {
    Solution::minimum_length("ca".into());
}

struct Solution {}

impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let mut i = 0;
        let mut j = s.len() - 1;
        while i < j && s[i] == s[j] {
            let c = s[i];
            while i <= j && s[i] == c { i += 1 }
            while i <= j && s[j] == c { j -= 1 }
        }
        (j + 1 - i) as i32 
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::minimum_length("ca".into()), 2);
        assert_eq!(Solution::minimum_length("ab".into()), 2);
        assert_eq!(Solution::minimum_length("cabaabac".into()), 0);
        assert_eq!(Solution::minimum_length("aabccabba".into()), 3);
        assert_eq!(Solution::minimum_length("bbab".into()), 1);
        assert_eq!(Solution::minimum_length("bbbb".into()), 0);
    }
}