use std::char::ParseCharError;

fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        let mut words = std::collections::HashMap::new();
        for word in paragraph.split(['!', '?', ' ', '\'', ',', '.', ';']) {
            if word.len() == 0 { continue }
            let word = word.to_lowercase();
            if banned.contains(&word) { continue }
            let x = words.entry(word).or_insert(0);
            *x += 1;
        }
        if let Some((a,_)) = words.iter().max_by_key(|x| x.1) {
            a.to_owned()
        } else { unimplemented!() }
    }
}

#[cfg(test)]
mod test {
    use std::vec;

    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::most_common_word(
            "Bob hit a ball, the hit BALL flew far after it was hit.".to_owned(),
            vec!["hit".to_owned()]
        ),"ball");
    }
}