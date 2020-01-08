fn main() {
    let mut s = String::from("123");
    s.push_str("123");
    println!("{:?}", s);

    let mut vec = Vec::new();
    vec.push(String::from("123"));
    vec[0].push_str("2");
    vec.push(String::from("123"));
    println!("{:?}", vec);

    Solution::convert(String::from("0123456789"), 3);

}

pub struct Solution {}

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if s.len() <= num_rows as usize || num_rows == 1 {
            return s;
        }
        let s_iter = s.chars();
        let mut cnt = 0;
        let mut vec = Vec::new();
        let mut up = false;
        let group = num_rows - 1;
        for c in s_iter {
            if cnt < num_rows {
                vec.push(c.to_string());
            } else {
                if cnt % group == 0 {
                    if !up {
                        vec[0].push(c);
                        up = true;
                    } else if up {
                        vec[group as usize].push(c);
                        up = false;
                    }
                }
                else if !up {
                    vec[(group - cnt % group) as usize].push(c);
                } else if up {
                    vec[(cnt % group) as usize].push(c);
                }
            }
            cnt += 1;
        }
        let mut ret = String::new();
        for i in 0..num_rows {
            ret.push_str(&vec[i as usize]);
        }
        ret
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::convert(String::from("PAYPALISHIRING"), 3), "PAHNAPLSIIGYIR");
        assert_eq!(Solution::convert(String::from("PAYPALISHIRING"), 4), "PINALSIGYAHRPI");
        assert_eq!(Solution::convert(String::from("123"), 4), "123");
        assert_eq!(Solution::convert(String::from("123"), 1), "123");
    }
}