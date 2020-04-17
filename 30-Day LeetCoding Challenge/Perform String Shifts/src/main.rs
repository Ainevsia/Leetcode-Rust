fn main() {
    println!("-1 % 5 = {:#?}", -1 % 5);
    println!("-6 % 5 = {:#?}", -6 % 5);
    println!("-11 % -5 = {:#?}", -11 % -5);
    println!("1 % -5 = {:#?}", 1 % -5);
    println!("6 % -5 = {:#?}", 6 % -5);
}

struct Solution {}

impl Solution {
    pub fn string_shift(s: String, shift: Vec<Vec<i32>>) -> String {
        let mut left_shift = 0;
        let rmd = s.len();
        for i in shift {
            let direction = i[0];
            let shift_amount = i[1];
            left_shift += if direction == 0 { shift_amount } else { -shift_amount };
        }
        let left_shift = if left_shift < 0 {
            (left_shift % rmd as i32 + rmd as i32) as usize
        } else {
            left_shift as usize % rmd
        };
        Self::shift_left(s, left_shift)
    }

    pub fn shift_left(s: String, shift: usize) -> String {
        if shift == 0 { return s }
        let s = s.chars().collect::<Vec<char>>();
        let mut res = vec![];
        for i in shift..s.len() {
            res.push(s[i]);
        }
        for i in 0..shift {
            res.push(s[i]);
        }
        res.iter().collect()
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::string_shift(
            "abcdefg".to_string(),
            vec![vec![1,1],vec![1,1],vec![0,2],vec![1,3]]
        ), "efgabcd");
    }
}