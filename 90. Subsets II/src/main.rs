fn main() {
    let mut a = vec![1,2,3];
    for i in 0..a.len() {
        println!("a = {:#?}", a);
        a.push(i);
    }
    assert_eq!(Solution::subsets_with_dup(vec![1,2,2]), vec![
        vec![2],
        vec![1],
        vec![1,2,2],
        vec![2,2],
        vec![1,2],
        vec![]
    ]);
}

struct Solution {}

impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![vec![]];
        nums.sort();
        let mut i = 0;
        while i < nums.len() {
            let mut cnt = 0;
            while i + cnt < nums.len() && nums[i + cnt] == nums[i] {
                cnt += 1;
            }
            for j in 0..res.len() {
                let mut a = res[j].clone();
                for _ in 0..cnt {
                    a.push(nums[i]);
                    res.push(a.clone());
                }
            }
            i += cnt;
        }
        res
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::subsets_with_dup(vec![1,2,2]), vec![
            vec![2],
            vec![1],
            vec![1,2,2],
            vec![2,2],
            vec![1,2],
            vec![]
        ]);
    }
}