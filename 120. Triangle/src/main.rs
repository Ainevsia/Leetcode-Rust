fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
        let a = triangle.len();
        for j in 1..a {
            triangle[j][0] += triangle[j - 1][0];
            for i in 1..j {
                // println!("i = {:#?}, j = {:#?}", i, j);
                // println!("triangle = {:#?}", triangle);
                // println!("triangle[j - 1][i - 1] = {:#?}", triangle[j - 1][i - 1]);
                // println!("triangle[j - 1][i] = {:#?}", triangle[j - 1][i]);
                triangle[j][i] += std::cmp::min(triangle[j - 1][i - 1], triangle[j - 1][i]);
                // println!("heree");
            }
            triangle[j][j] += triangle[j - 1][j - 1];
            // println!("triangle = {:?}", triangle);
        }
        let mut minx = std::i32::MAX;
        for &i in &triangle[a - 1] {
            if i < minx {
                minx = i;
            }
        }
        minx
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::minimum_total(vec![
            vec![2],
           vec![3,4],
          vec![6,5,7],
         vec![4,1,8,3]
       ]), 11);
    }
}