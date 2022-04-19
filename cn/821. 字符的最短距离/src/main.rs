use std::vec;

fn main() {
    Solution::shortest_to_char("loveleetcode".to_owned(), 'e');
}

struct Solution {}

impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let x = s.chars().enumerate().filter_map(|(x,y)| {
            if y == c { Some(x as i32) } else { None }
        }).collect::<Vec<_>>();
        let mut res = Vec::with_capacity(s.len());
        
        for i in (0..=x[0]).rev() { res.push(i); }
        for i in 0..x.len()-1 { Self::add(&mut res, x[i+1]-x[i]); }
        let remains = s.len() as i32 - *x.last().unwrap();
        for i in 1..remains { res.push(i); }
        res
    }

    /// n == 2, [1,0]
    /// n == 3, [1,1,0]
    /// n == 4, [1,2,1,0]
    /// n == 5, [1,2,2,1,0]
    fn add(res: &mut Vec<i32>, n: i32) {
        if n == 1 { res.push(0); return; }
        let mid = n >> 1;
        for i in 1..=mid { res.push(i); }
        if n & 1 == 1 { res.push(mid); }
        for i in (0..mid).rev() { res.push(i); }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::shortest_to_char("loveleetcode".to_owned(), 'e'),vec![3,2,1,0,1,0,0,1,2,2,1,0]);
        assert_eq!(Solution::shortest_to_char("aaab".to_owned(), 'b'),vec![3,2,1,0]);
        assert_eq!(Solution::shortest_to_char("aaaba".to_owned(), 'b'),vec![3,2,1,0,1]);
        assert_eq!(Solution::shortest_to_char("a".to_owned(), 'a'),vec![0]);
    }
}