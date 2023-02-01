pub struct Solution {}

impl Solution {
    pub fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        for k in knowledge.iter() {
            let (key, value) = (&k[0][..], &k[1][..]);
            map.entry(key).or_insert(value);
        }
        let mut res = String::new();
        let mut append = true;
        for s in s.split(&['(', ')'][..]) {
            if append { 
                res.push_str(s);
            } else {
                if map.get(s).is_none() {
                    res.push_str("?");
                } else {
                    res.push_str(map.entry(s).or_default());
                }
            }
            append = ! append;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::evaluate("(name)is(age)yearsold".into(), 
                vec![vec!["name".into(),"bob".into()],vec!["age".into(),"two".into()]]
            ), "bobistwoyearsold".to_string()
        );
    }
}
