fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut node_exist = vec![true; num_courses as usize];
        let mut edges_from = vec![vec![]; num_courses as usize];
        let mut in_degrees = vec![0; num_courses as usize];
        for vec in prerequisites {
            edges_from[vec[1] as usize].push(vec[0] as usize);
            in_degrees[vec[0] as usize] += 1;
        }
        let mut s = vec![];
        for i in 0..num_courses as usize {
            if in_degrees[i] == 0 { s.push(i) }
        }
        let mut buf = vec![];
        while !s.is_empty() {
            let src = s.pop().unwrap();
            node_exist[src] = false;
            for &i in edges_from[src].iter() {
                if node_exist[i] {
                    in_degrees[i] -= 1;
                    if in_degrees[i] == 0 { s.push(i) }
                }
            }
            buf.push(src as i32);
        }
        if node_exist.iter().all(|&x| x == false) {
            buf
        } else { vec![] }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::find_order(2, vec![vec![1,0]]), vec![0,1]);
        assert_eq!(Solution::find_order(4, vec![vec![1,0],vec![2,0],vec![3,1],vec![3,2]]), vec![0,2,1,3]);
    }
}