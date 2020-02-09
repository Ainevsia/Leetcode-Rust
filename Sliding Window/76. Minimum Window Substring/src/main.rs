fn main() {
    Solution::min_window(String::from("ADOBECODEBANC"), String::from("ABCDA"));
}

struct Solution {}

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let s: Vec<char> = s.chars().collect();
        use std::collections::HashMap;
        let mut map: HashMap<char, i32> = HashMap::new();
        for c in t.chars() {
            map.entry(c).and_modify(|x| *x += 1 ).or_insert(1);
        }
        println!("m = {:#?}", map);
        let (mut start, mut end) = (0, 0);
        let mut cnt = t.len();
        let mut wnd = usize::max_value();
        let mut wnd_start = 0;
        while end < s.len() {
            println!("start = {:#?}, end = {:#?}", start, end);
            println!("s[start..end] = {:?}", &s[start..=end]);
            map.entry(s[end]).and_modify(|x| {
                if *x > 0 { cnt -= 1; }
                *x -= 1;
            });
            println!("m = {:#?}", map);
            end += 1;
            while cnt == 0 {
                if end - start < wnd {
                    wnd = end - start;
                    wnd_start = start;
                }
                map.entry(s[start]).and_modify(|x| {
                    *x += 1;
                    if *x > 0 { cnt += 1; }
                });
                start += 1;
            }
        }
        if wnd == usize::max_value() { return "".to_string() }
        let res: String = (&s[wnd_start..wnd_start+wnd]).to_owned().iter().collect();
        println!("res = {:?}", res);
        res
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::min_window(String::from("ADOBECODEBANC"), String::from("ABC")), String::from("BANC"));
        assert_eq!(Solution::min_window(String::from("ADOBECODEBANC"), String::from("ABCD")), String::from("ADOBEC"));
        assert_eq!(Solution::min_window(String::from("ADOBECODEBANC"), String::from("ABF@CD")), String::from(""));


    }
}
