fn main() {
    Solution::interval_intersection(vec![], vec![]);
    match 1.cmp(&2) {
        Equal | Greater => unreachable!(),
        _ => unreachable!()
    }
}

struct Solution {}

use std::cmp::Ordering::{Equal, Greater, Less};

impl Solution {
    pub fn interval_intersection(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if a.len() == 0 || b.len() == 0 { return vec![] }
        let mut res = vec![];
        let (mut i, mut j) = (0, 0);
        while i < a.len() && j < b.len() {
            // six different cases
            match (a[i][0].cmp(&b[j][0]), a[i][1].cmp(&b[j][1]), a[i][0].cmp(&b[j][1]), a[i][1].cmp(&b[j][0])) {
                (_, _, _, Less) => i += 1,
                (_, _, Greater, _) => j += 1,
                (Greater, Less, _, _) | (Equal, Less, _, _) | (Greater, Equal, _, _) | (Equal, Equal, _, _) =>
                    { res.push(a[i].clone()); i += 1 }
                (Less, Greater, _, _) | (Less, Equal, _, _) | (Equal, Greater, _, _) =>
                    { res.push(b[j].clone()); j += 1 }
                (Less, Less, _, Greater) | (Less, Less, _, Equal) => 
                    { res.push(vec![b[j][0], a[i][1]]); i += 1 }
                (Greater, Greater, Less, _) | (Greater, Greater, Equal, _) => 
                    { res.push(vec![a[i][0], b[j][1]]); j += 1 }
            }
        }
        res
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::interval_intersection(
            vec![vec![0,2],vec![5,10],vec![13,23],vec![24,25]],
            vec![vec![1,5],vec![8,12],vec![15,24],vec![25,26]]
        ), vec![vec![1,2],vec![5,5],vec![8,10],vec![15,23],vec![24,24],vec![25,25]]);
    }
}