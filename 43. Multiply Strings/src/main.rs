fn main() {
    Solution::multiply("231".to_string(), "32".to_string());
}

struct Solution {}

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let num1: Vec<u32> = num1.bytes().map(|x| (x - b'0') as u32).collect();
        let num2: Vec<u32> = num2.bytes().map(|x| (x - b'0') as u32).collect();
        let mut res = vec![0u32; num1.len() + num2.len()];
        for j in 0..num2.len() {
            for i in 0..num1.len() {
                let sum = num1[i] * num2[j];
                res[i + j + 1] += sum % 10;
                res[i + j] += sum / 10;
            }
        }
        for i in (1..res.len()).rev() {
            res[i - 1] += res[i] / 10;
            res[i] = res[i] % 10;
        }
        let mut res: String = res.iter().fold(String::from(""),
            |mut acc, &x| {
                acc.push((x as u8 + '0' as u8) as char);
                acc
            });
        while !res.is_empty() && res.chars().nth(0) == Some('0') {
            res.remove(0);
        }
        println!("res = {:#?}", res);
        if res.is_empty() { String::from("0") } else { res }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::multiply("2".to_string(), "3".to_string()), "6".to_string());
        assert_eq!(Solution::multiply("123".to_string(), "456".to_string()), "56088".to_string());
        assert_eq!(Solution::multiply("0".to_string(), "456".to_string()), "0".to_string());
        assert_eq!(Solution::multiply("123456789".to_string(), "987654321".to_string()), "121932631112635269".to_string());
        assert_eq!(Solution::multiply("401716832807512840963".to_string(), "167141802233061013023557397451289113296441069".to_string()), "67143675422804947379429215144664313370120390398055713625298709447".to_string());
        
    }
}