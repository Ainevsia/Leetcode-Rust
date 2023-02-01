fn main() {
    Solution::minimum_moves("XXX".into());
    let string = "1234567";
    let mut chars = string.chars();
    let sub_string = (0..)
        .map(|_| chars.by_ref().take(2).collect::<String>())
        .take_while(|s| !s.is_empty())
        .collect::<Vec<_>>();
    let words: [&str; 4] = ["hello", "world", "of", "Rust"];
    let mut words: std::array::IntoIter<&str, 4> = words.into_iter();

    let a: &mut std::array::IntoIter<&str, 4> = words.by_ref();
    let b = words.take(3);
    let c = a.take(3);
    // Take the first two words.
    let hello_world: Vec<_> = a.take(2).collect();
    assert_eq!(hello_world, vec!["hello", "world"]);
    
    // Collect the rest of the words.
    // We can only do this because we used `by_ref` earlier.
    // let of_rust: Vec<_> = words.collect();
    // assert_eq!(of_rust, vec!["of", "Rust"]);
}

struct Solution {}

impl Solution {
    pub fn minimum_moves(s: String) -> i32 {
        let mut s = s.chars();
        let mut res = 0;
        while let Some(x) = s.next() {
            if x == 'X' {
                s.next();
                s.next();
                res += 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::minimum_moves("XXX".into()), 1);
        assert_eq!(Solution::minimum_moves("XXOX".into()), 2);
        assert_eq!(Solution::minimum_moves("OOOO".into()), 0);
    }
}