fn main() {
    let mut vec = vec![vec![2], vec![1,2]];
    vec.sort();
    println!("vec = {:#?}", vec);
    assert_eq!(Solution::merge(vec![vec![1,4],vec![4,5]]), vec![vec![1,5]]);
}

struct Solution {}

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.len() <= 1 { return intervals }
        intervals.sort();
        let (mut start, mut end) = (intervals[0][0], intervals[0][1]);
        let mut res = vec![];
        for i in 1..intervals.len() {
            if intervals[i][1] <= end { continue }
            else if intervals[i][0] > end {
                res.push(vec![start, end]);
                start = intervals[i][0];
                end = intervals[i][1];
            } else {
                end = intervals[i][1];
            }
        }
        res.push(vec![start, end]);
        res
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::merge(vec![vec![1,3],vec![2,6],vec![8,10],vec![15,18]]), vec![vec![1,6],vec![8,10],vec![15,18]]);
        assert_eq!(Solution::merge(vec![vec![1,4],vec![4,5]]), vec![vec![1,5]]);
        assert_eq!(Solution::merge(vec![vec![1,4],vec![4,5],vec![5,6]]), vec![vec![1,6]]);

    }
}