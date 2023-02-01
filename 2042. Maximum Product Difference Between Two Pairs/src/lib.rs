pub struct Solution {}

impl Solution {
    pub fn are_numbers_ascending(s: String) -> bool {
        let mut prev = 0;
        for s in s.split(' ') {
            if s.starts_with(char::is_numeric) {
                let s: u32 = s.parse().unwrap();
                if s <= prev { return false }
                else { prev = s }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::are_numbers_ascending(
            "1 box has 3 blue 4 red 6 green and 12 yellow marbles"
            .into()), true);
        assert_eq!(Solution::are_numbers_ascending(
            "hello world 5 x 5"
            .into()), false);
        assert_eq!(Solution::are_numbers_ascending(
            "sunset is at 7 51 pm overnight lows will be in the low 50 and 60 s"
            .into()), false);
        assert_eq!(Solution::are_numbers_ascending(
            "4 5 11 26"
            .into()), true);
    }
}
