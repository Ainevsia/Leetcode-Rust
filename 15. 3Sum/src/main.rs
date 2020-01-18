fn main() {
    Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]);
    let mut x = vec![vec![5,5,3,2,1],vec![5,5,3,2,1]];
    x.sort();
    x.dedup();
    println!("x = {:?}", x);
}

struct Solution {}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums.clone();
        nums.sort();
        let target: Vec<i32> = nums.clone()
                                   .iter().rev().map(|&x| x)
                                   .collect();
        // println!("nums = {:?}, target = {:?}", nums, target);
        let mut ret: Vec<Vec<i32>> = Vec::new();
        let (mut n_iter, mut t_iter) = (nums.iter(), target.iter());
        let (mut x, mut y, mut z) = (n_iter.next(), n_iter.next(), t_iter.next());
        let (mut id1, mut id2, mut id3) = (1, 2, nums.len());
        ret = loop {
            println!("x = {:?}, y = {:?}, z = {:?}", x, y, z);
            println!("id1 = {:?}, id2 = {:?}, id3 = {}", id1, id2, id3);
            if let (Some(&a), Some(&b), Some(&c)) = (x, y, z) {
                if a + b + c < 0 {
                    x = y;
                    y = n_iter.next();
                    id1 += 1;
                    id2 += 1
                } else if a + b + c > 0 || id1 == id3 || id2 == id3 {
                    z = t_iter.next();
                    id3 -= 1
                } else if a + b + c == 0 {
                    println!("id1 = {:?}, id2 = {:?}, id3 = {}", id1, id2, id3);
                    println!("a = {:?}, b = {:?}, c = {}", a, b, c);
                    let mut new_vec = vec![a, b, c];
                    new_vec.sort();
                    ret.push(new_vec);
                    x = y;
                    y = n_iter.next();
                    id1 += 1;
                    id2 += 1
                }
            } else {
                break ret
            }
        };
        ret.dedup();
        ret
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn basic() {
        // assert_eq!(
        //     Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
        //     vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        // );
        // let a: Vec<Vec<i32>> = Vec::new();
        // assert_eq!(
        //     Solution::three_sum(vec![0, 0]),
        //     a
        // );
        // assert_eq!(
        //     Solution::three_sum(vec![-1,0,1,0]),
        //     vec![vec![-1,0,1]]
        // );
        assert_eq!(
            Solution::three_sum(vec![3,0,-2,-1,1,2]),
            vec![vec![-2,-1,3], vec![-2,0,2], vec![-1,0,1]]
        );
    }
}