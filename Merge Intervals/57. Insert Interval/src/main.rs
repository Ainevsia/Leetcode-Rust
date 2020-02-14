fn main() {
    assert_eq!(Solution::insert(vec![vec![1,3],vec![6,9]], vec![2,5]), vec![vec![1,5],vec![6,9]]);
}

struct Solution {}

impl Solution {
    pub fn insert_inplace(mut intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut i = 0;
        while i < intervals.len() {
            if intervals[i][1] < new_interval[0] { i += 1; continue }
            if intervals[i][0] > new_interval[1] { intervals.insert(i, new_interval); return intervals }
            if intervals[i][0] < new_interval[0] { new_interval[0] = intervals[i][0]; }
            if intervals[i][1] > new_interval[1] { new_interval[1] = intervals[i][1]; }
            intervals.remove(i);
        }
        intervals.push(new_interval);
        intervals
    }

    pub fn insert(mut intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut i = 0;
        let mut res = vec![];
        while i < intervals.len() {
            if intervals[i][1] < new_interval[0] { res.push(intervals[i].to_owned()); i += 1; continue  }
            if intervals[i][0] > new_interval[1] { break }
            if intervals[i][0] < new_interval[0] { new_interval[0] = intervals[i][0]; }
            if intervals[i][1] > new_interval[1] { new_interval[1] = intervals[i][1]; }
            i += 1;
        }
        res.push(new_interval);
        res.append(&mut intervals[i..].to_owned());
        res
    }

}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::insert(vec![vec![1,3],vec![6,9]], vec![2,5]), vec![vec![1,5],vec![6,9]]);
        assert_eq!(Solution::insert(vec![vec![1,2],vec![3,5],vec![6,7],vec![8,10],vec![12,16]], vec![4,8]), vec![vec![1,2],vec![3,10],vec![12,16]]);
        assert_eq!(Solution::insert(vec![vec![1,2]], vec![4,8]), vec![vec![1,2],vec![4,8]]);
        assert_eq!(Solution::insert(vec![], vec![4,8]), vec![vec![4,8]]);

    }
}