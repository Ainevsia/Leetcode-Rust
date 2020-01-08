fn main() {
    println!("Hello, world!");
}

pub struct Solution {}

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        println!("Hello, world!");
        s
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::convert(String::from("12"), 1), "12");
    }
}