fn main() {
    assert_eq!(Solution::can_finish(2, vec![vec![1,0]] ), true);
}

struct Solution {}

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut edges_from = vec![vec![]; num_courses as usize];
        let mut node_exist = vec![true; num_courses as usize];
        let mut in_degrees = vec![0; num_courses as usize];
        for x in prerequisites {
            edges_from[x[1] as usize].push(x[0] as usize);
            in_degrees[x[0] as usize] += 1;
        }
        let mut s = vec![];
        // prepare stack
        for i in 0..edges_from.len() {
            if in_degrees[i] == 0 { s.push(i) }
        }
        while !s.is_empty() {
            let src = s.pop().unwrap();
            node_exist[src] = false;
            for &v in edges_from[src].iter() {
                if node_exist[v] {
                    in_degrees[v] -= 1;
                    if in_degrees[v] == 0 { s.push(v) }
                }
            }
        }
        node_exist.iter().all(|&x| x == false)
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