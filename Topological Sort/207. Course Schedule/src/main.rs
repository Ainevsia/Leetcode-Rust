fn main() {
    assert_eq!(Solution::can_finish(2, vec![vec![1,0]] ), true);
}

struct Solution {}

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut in_degrees = vec![vec![]; num_courses as usize];
        let mut node_exist = vec![true; num_courses as usize];
        for x in prerequisites {
            in_degrees[x[0] as usize].push(x[1] as usize);
        }
        let mut s = vec![];
        // prepare stack
        for i in 0..in_degrees.len() {
            if in_degrees[i].len() == 0 { s.push(i) }
        }
        while !s.is_empty() {
            let src = s.pop().unwrap();
            node_exist[src] = false;
            in_degrees = in_degrees.into_iter().enumerate().map(|(i, x)| {
                let tmp: Vec<usize> = x.into_iter().filter(|&x| x != src).collect();
                if tmp.len() == 0 && node_exist[i] {
                    s.push(i);
                    node_exist[i] = false
                }
                tmp
            }).collect();
        }
        in_degrees.iter().all(|x| x.is_empty())
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::can_finish(2, vec![vec![1,0]] ), true);
        assert_eq!(Solution::can_finish(2, vec![vec![1,0],vec![0,1]] ), false);
    }
}