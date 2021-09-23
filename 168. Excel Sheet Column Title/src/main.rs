fn main() {
    assert_eq!(Solution::convert_to_title(701), "A".to_owned());

}

struct Solution {}

impl Solution {
    // transfer into 26-radix number
    pub fn convert_to_title(column_number: i32) -> String {
        let mut res = vec![];
        let mut num = column_number as usize;
        while num != 0 {
            let reminder = if num % 26 == 0 { 26 } else { num % 26 };
            res.push(reminder as u8 + 'A' as u8 - 1);
            // num must be a multiple of 26 now
            num = (num - reminder) / 26;
        }
        res.reverse();
        String::from_utf8(res).unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::convert_to_title(1), "A".to_owned());
        assert_eq!(Solution::convert_to_title(28), "AB".to_owned());
        assert_eq!(Solution::convert_to_title(701), "ZY".to_owned());
        assert_eq!(Solution::convert_to_title(2147483647), "FXSHRXW".to_owned());

    }
}