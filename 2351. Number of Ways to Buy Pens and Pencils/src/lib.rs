pub struct Solution {}

impl Solution {
    pub fn repeated_character(s: String) -> char {
        let mut v = vec![0; 256];
        for c in s.chars() {
            let i = c as usize;
            dbg!(i);
            if v[i] == 1 { return c }
            else { v[i] = 1 }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        Solution::repeated_character("abcdsa".into());
        let result = 2 + 2;
        assert_eq!(result, 5);
    }
}
