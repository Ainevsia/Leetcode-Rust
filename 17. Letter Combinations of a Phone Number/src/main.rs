fn main() {
    Solution::letter_combinations("23".to_string());
}

struct Solution {}

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut old;
        let mut new: Vec<String> = vec![];
        for d in digits.chars() {
            old = new;
            new = Vec::<String>::new();
            match d {
                '2' => Solution::permute_once(&old, &mut new, &vec!["a","b","c"]),
                '3' => Solution::permute_once(&old, &mut new, &vec!["d","e","f"]),
                '4' => Solution::permute_once(&old, &mut new, &vec!["g","h","i"]),
                '5' => Solution::permute_once(&old, &mut new, &vec!["j","k","l"]),
                '6' => Solution::permute_once(&old, &mut new, &vec!["m","n","o"]),
                '7' => Solution::permute_once(&old, &mut new, &vec!["p","q","r","s"]),
                '8' => Solution::permute_once(&old, &mut new, &vec!["t","u","v"]),
                '9' => Solution::permute_once(&old, &mut new, &vec!["w","x","y","z"]),
                _ => (),
            }
        }
        new
    }

    fn permute_once(old: &Vec<String>,
                new: &mut Vec<String>,
                list: &Vec<&str>) {
        if old.len() == 0 {
            for &c in list.iter() {
                new.push(String::from(format!("{}", c)));
            }
            return;
        }
        for item in old.iter() {
            for &c in list.iter() {
                new.push(String::from(format!("{}{}", item, c)));
            }
        }
    }


    pub fn letter_combinations_(digits: String) -> Vec<String> {
        digits.chars().fold(
            vec!["".to_string()],
            |acc, digit| acc.iter().flat_map(
                |item| Self::get_char(digit).chars().map(
                    |c| format!("{}{}", item, c)
                ).collect::<Vec<String>>()
            ).collect()
        )
    }

    fn get_char(c: char) -> String {
        match c {
            '2' => "abc",
            '3' => "def",
            '4' => "ghi",
            '5' => "jkl",
            '6' => "mno",
            '7' => "pqrs",
            '8' => "tuv",
            '9' => "wxyz",
            _   => "",
        }.to_string()
        // into vs to_string
    }
}



#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(
            Solution::letter_combinations_("23".to_string()),
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"].iter().map(|x| x.to_string()).collect::<Vec<String>>()
        )
    }
}