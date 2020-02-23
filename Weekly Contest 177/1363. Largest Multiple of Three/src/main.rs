fn main() {
    for i in 0..100 {
        println!("3*i = {:#?}", 3*i);
    }
    assert_eq!(Solution::largest_multiple_of_three(vec![8,1,9]), "981");
}

struct Solution {}

impl Solution {
    pub fn largest_multiple_of_three(mut digits: Vec<i32>) -> String {
        let sum =  digits.iter().fold(0, |sum, &x| sum + x);
        digits.sort();
        digits = digits.into_iter().rev().collect();
        if sum % 3 != 0 {
            // println!("digits = {:#?}", digits);
            let mut m1 = 0;let mut m2 = 0;
            for &i in digits.iter() {
                if i % 3 == 1 { m1 += 1 }
                if i % 3 == 2 { m2 += 1 }
            }
            if m1 != 0 && m2 != 0 && i32::abs(m1-m2) % 3 == 1 && (m1 % 3 == 2 ||m2%3==2) {
                m1 -= 1; m2 -= 1;
            }
            m1 %= 3; m2 %= 3;
            while m1 != 0 && m2 != 0 { 
                m1 -= 1; m2 -= 1;
            }
            // println!("m1 = {:#?}, m2 = {:#?}", m1, m2);
            match (m1, m2) {
                (1, 0) => {
                    let mut j = digits.len() - 1;
                    loop { 
                        if digits[j] % 3 == 1 { digits.remove(j); break }
                        if j == 0 { return String::from("") }
                        else { j -= 1 }
                    }
                }
                (0, 1) => {
                    let mut j = digits.len() - 1;
                    loop { 
                        if digits[j] % 3 == 2 { digits.remove(j); break }
                        if j == 0 { return String::from("") }
                        else { j -= 1 }
                    }
                }
                (2, 0) => {
                    let mut j = digits.len() - 1;
                    let mut cnt = 2;
                    loop { 
                        if digits[j] % 3 == 1 {
                            digits.remove(j);
                            cnt -= 1;
                            if cnt == 0 { break }
                        }
                        if j == 0 { return String::from("") }
                        else { j -= 1 }
                    }
                }
                (0, 2) => {
                    let mut j = digits.len() - 1;
                    let mut cnt = 2;
                    loop { 
                        if digits[j] % 3 == 2 {
                            digits.remove(j);
                            cnt -= 1;
                            if cnt == 0 { break }
                        }
                        if j == 0 { return String::from("") }
                        else { j -= 1 }
                    }
                }
                _ => unimplemented!()
            }
        }
        if digits.len() == 0 { return "".to_string() }
        digits.sort();
        // println!("digits = {:#?}", digits);
        let mut res: Vec<i32> = digits.into_iter().rev().collect();
        while !res.is_empty() && res[0] == 0 { res.remove(0); }
        if res.is_empty() { return "0".to_string() }

        res.iter().map(|&x| std::char::from_digit(x as u32, 10).unwrap()).collect()
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::largest_multiple_of_three(vec![8,1,9]), "981");
        assert_eq!(Solution::largest_multiple_of_three(vec![8,6,7,1,0]), "8760");
        assert_eq!(Solution::largest_multiple_of_three(vec![1]), "");
        assert_eq!(Solution::largest_multiple_of_three(vec![0,0,0,0,0,0]), "0");

    }

    #[test]
    fn fail() {
        assert_eq!(Solution::largest_multiple_of_three(vec![2,2,1,1,1]), "2211");
        assert_eq!(Solution::largest_multiple_of_three(vec![1,1,1,2]), "111");
        assert_eq!(Solution::largest_multiple_of_three(vec![9,8,6,8,6]), "966");
        assert_eq!(Solution::largest_multiple_of_three(vec![7,8,7,2,1,2,6,6,9,2]), "987766221");
        assert_eq!(Solution::largest_multiple_of_three(vec![7,1,2,4,0,0,4,0,3,8]), "874431000");

    }
}