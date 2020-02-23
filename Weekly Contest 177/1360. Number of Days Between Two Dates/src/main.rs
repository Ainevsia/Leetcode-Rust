fn main() {
    assert_eq!(Solution::days_between_dates(String::from("2019-06-29"), String::from("2019-06-30")), 1);
}

struct Solution {}

/// The given dates are valid dates between the years 1971 and 2100.
/// i32 is enough
const MONTH: [i32; 12] = [31,28,31,30,31,30,31,31,30,31,30,31];

impl Solution {
    pub fn days_between_dates(date1: String, date2: String) -> i32 {
        i32::abs(Self::get(&date1) - Self::get(&date2))
    }

    pub fn get(d: &str) -> i32 {
        let y = d[0..4].parse::<i32>().unwrap();
        let m = d[5..7].parse::<i32>().unwrap();
        let d = d[8..].parse::<i32>().unwrap();
        let mut res = 0;
        for i in 1971..y {
            res += 365;
            if i % 4 == 0 && i % 100 != 0 || i % 400 == 0 {
                res += 1
            }
        }
        for i in 1..m {
            res += MONTH[i as usize - 1];
            if i == 2 && (y % 4 == 0 && y % 100 != 0 || y % 400 == 0) {
                res += 1
            }
        }
        res + d
    }
}


#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::days_between_dates(String::from("2019-06-29"), String::from("2019-06-30")), 1);
        assert_eq!(Solution::days_between_dates(String::from("2020-01-15"), String::from("2019-12-31")), 15);

    }
}