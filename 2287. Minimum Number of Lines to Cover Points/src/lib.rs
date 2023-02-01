pub struct Solution {}

impl Solution {
    pub fn rearrange_characters(s: String, target: String) -> i32 {
        use std::collections::HashMap;
        let mut map: HashMap<char, i32> = HashMap::new();
        for ch in s.chars() {
            *map.entry(ch).or_default() += 1;
        }
        let mut map2: HashMap<char, i32> = HashMap::new();
        for ch in target.chars() {
            *map2.entry(ch).or_default() += 1;
        }
        let mut res = i32::MAX;
        for (ch, cnt) in map2 {
            let a = *map.entry(ch).or_default();
            let b = a / cnt;
            if b < res { res = b }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::rearrange_characters("ilovecodingonleetcode".into(), "code".into()), 2);
        assert_eq!(Solution::rearrange_characters("abbaccaddaeea".into(), "aaaaa".into()), 1);
    }
}
